// Adapted from ArialSenki MultipleSpiritAshes; MIT OR Apache-2.0.
#![allow(dead_code)]
//! Four individual Spirit rows in the loaded HUD movie, adapted from upstream's parser.
//! Only RemotePCTag0..3 are repositioned. Their shared row sprite is cloned
//! before icon changes, preserving the template used by the other native tags.
//! All required layout changes succeed together or original bytes are served.
//! Buddy is hidden only in the successful replacement movie. This prototype
//! requires restart with EnableGroupHud=false for recovery after a runtime fault.

use std::{
    collections::HashMap,
    ffi::c_void,
    sync::{
        LazyLock, Mutex,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
};

/// Patch values (this mod's own HUD layout).
const ROW_SCALE_FIXED: i32 = 53_739; // 0.82 in 16.16 fixed point
const ROW_SPACING_TWIPS: i32 = 900; // 45 px
/// Moves the rows left so their name and bar start at the HUD's left edge,
/// with the portrait frame and the quick slots (user's screenshot, 24-10:
/// text at x 147 of 2000 -> 52; -91 HUD px at scale 1).
const ROW_SHIFT_X_TWIPS: i32 = -1820;
/// Moves the block up from the Buddy panel into the free space under the
/// status icons, clear of the game's action prompt ("Kick") and the quick
/// slots (user's screenshot, 24-10: first row at y 565 of 1125 -> ~200;
/// -350 HUD px at scale 1).
const ROW_SHIFT_Y_TWIPS: i32 = -7000;
/// The game has seven friendly tag entries and binds seven tags,
/// RemotePCTag0..6, but its movie only places six: the seventh is added.
const ROW_COUNT: usize = crate::hud::MAX_ROWS;
const MOVIE_ROW_TAGS: usize = 6;
const HIDDEN_ROW_ICONS: [&[u8]; 4] = [
    b"RankIcon",
    b"BigRune",
    b"VoiceChatIconRoot",
    b"MultiPlayWeakIcon",
];
const ROW_NAME: &[u8] = b"Name";
const DIMMED_ROW_NAME: &[u8] = b"RoleName";
const DIMMED_ALPHA: i32 = 128; // 0.5 in 8.8 fixed point
const BUDDY_PANEL: &[u8] = b"Buddy";
const ROW_TAG_PREFIX: &[u8] = b"RemotePCTag";
pub(crate) const HUD_MOVIE_FILE: &str = "01_000_fe.gfx";

const TAG_END: u16 = 0;
const TAG_PLACE_OBJECT2: u16 = 26;
const TAG_DEFINE_SPRITE: u16 = 39;

// ---------------------------------------------------------------------------
// GFX (uncompressed SWF container) model
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
struct Tag {
    code: u16,
    long_header: bool,
    body: Vec<u8>,
}

fn parse_tags(data: &[u8], mut pos: usize) -> Result<Vec<Tag>, &'static str> {
    let mut tags = Vec::new();
    while pos < data.len() {
        let header = read_u16(data, pos).ok_or("truncated tag header")?;
        let code = header >> 6;
        let mut len = usize::from(header & 0x3f);
        let long_header = len == 0x3f;
        pos += 2;
        if long_header {
            len = read_u32(data, pos).ok_or("truncated long tag header")? as usize;
            pos += 4;
        }
        let end = pos.checked_add(len).ok_or("tag length overflow")?;
        let body = data
            .get(pos..end)
            .ok_or("tag runs past the end of the movie")?;
        tags.push(Tag {
            code,
            long_header,
            body: body.to_vec(),
        });
        pos = end;
        if code == TAG_END {
            break;
        }
    }
    Ok(tags)
}

fn write_tags(out: &mut Vec<u8>, tags: &[Tag]) -> Result<(), &'static str> {
    for tag in tags {
        let len = tag.body.len();
        if tag.long_header || len >= 0x3f {
            let len = u32::try_from(len).map_err(|_| "tag too large")?;
            out.extend_from_slice(&((tag.code << 6) | 0x3f).to_le_bytes());
            out.extend_from_slice(&len.to_le_bytes());
        } else {
            out.extend_from_slice(&((tag.code << 6) | len as u16).to_le_bytes());
        }
        out.extend_from_slice(&tag.body);
    }
    Ok(())
}

fn read_u16(data: &[u8], pos: usize) -> Option<u16> {
    Some(u16::from_le_bytes(data.get(pos..pos + 2)?.try_into().ok()?))
}

fn read_u32(data: &[u8], pos: usize) -> Option<u32> {
    Some(u32::from_le_bytes(data.get(pos..pos + 4)?.try_into().ok()?))
}

struct BitReader<'a> {
    data: &'a [u8],
    bit: usize,
}

impl<'a> BitReader<'a> {
    fn new(data: &'a [u8], byte: usize) -> Self {
        Self {
            data,
            bit: byte * 8,
        }
    }

    fn unsigned(&mut self, count: u32) -> Option<u32> {
        let mut value = 0u32;
        for _ in 0..count {
            let byte = *self.data.get(self.bit >> 3)?;
            value = (value << 1) | u32::from((byte >> (7 - (self.bit & 7))) & 1);
            self.bit += 1;
        }
        Some(value)
    }

    fn signed(&mut self, count: u32) -> Option<i32> {
        let value = self.unsigned(count)?;
        if count == 0 {
            return Some(0);
        }
        let shift = 32 - count;
        Some(((value << shift) as i32) >> shift)
    }

    fn byte_pos(&self) -> usize {
        self.bit.div_ceil(8)
    }
}

#[derive(Default)]
struct BitWriter {
    out: Vec<u8>,
    bit: usize,
}

impl BitWriter {
    fn put(&mut self, value: u32, count: u32) {
        for index in (0..count).rev() {
            if self.bit % 8 == 0 {
                self.out.push(0);
            }
            if (value >> index) & 1 == 1 {
                let last = self.out.len() - 1;
                self.out[last] |= 0x80 >> (self.bit % 8);
            }
            self.bit += 1;
        }
    }

    fn put_signed(&mut self, value: i32, count: u32) {
        self.put(value as u32 & mask(count), count);
    }
}

fn mask(count: u32) -> u32 {
    if count >= 32 {
        u32::MAX
    } else {
        (1u32 << count) - 1
    }
}

/// Smallest signed bit width that holds every value.
fn signed_bits(values: &[i32]) -> u32 {
    values
        .iter()
        .map(|&value| {
            if value == 0 {
                0
            } else if value > 0 {
                33 - value.leading_zeros()
            } else {
                33 - (!value).leading_zeros()
            }
        })
        .max()
        .unwrap_or(0)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Matrix {
    scale: Option<(i32, i32)>,
    rotate: Option<(i32, i32)>,
    translate: (i32, i32),
}

impl Matrix {
    fn read(data: &[u8], pos: usize) -> Option<(Self, usize)> {
        let mut bits = BitReader::new(data, pos);
        let scale = if bits.unsigned(1)? == 1 {
            let count = bits.unsigned(5)?;
            Some((bits.signed(count)?, bits.signed(count)?))
        } else {
            None
        };
        let rotate = if bits.unsigned(1)? == 1 {
            let count = bits.unsigned(5)?;
            Some((bits.signed(count)?, bits.signed(count)?))
        } else {
            None
        };
        let count = bits.unsigned(5)?;
        let translate = (bits.signed(count)?, bits.signed(count)?);
        Some((
            Self {
                scale,
                rotate,
                translate,
            },
            bits.byte_pos(),
        ))
    }

    fn write(&self, out: &mut Vec<u8>) -> Option<()> {
        let mut bits = BitWriter::default();
        for pair in [self.scale, self.rotate] {
            match pair {
                Some((a, b)) => {
                    let count = signed_bits(&[a, b]);
                    if count > 31 {
                        return None;
                    }
                    bits.put(1, 1);
                    bits.put(count, 5);
                    bits.put_signed(a, count);
                    bits.put_signed(b, count);
                }
                None => bits.put(0, 1),
            }
        }
        let (x, y) = self.translate;
        let count = signed_bits(&[x, y]);
        if count > 31 {
            return None;
        }
        bits.put(count, 5);
        bits.put_signed(x, count);
        bits.put_signed(y, count);
        out.extend_from_slice(&bits.out);
        Some(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ColorTransform {
    mult: Option<[i32; 4]>,
    add: Option<[i32; 4]>,
}

impl ColorTransform {
    fn read(data: &[u8], pos: usize) -> Option<(Self, usize)> {
        let mut bits = BitReader::new(data, pos);
        let has_add = bits.unsigned(1)? == 1;
        let has_mult = bits.unsigned(1)? == 1;
        let count = bits.unsigned(4)?;
        let read4 = |bits: &mut BitReader| -> Option<[i32; 4]> {
            Some([
                bits.signed(count)?,
                bits.signed(count)?,
                bits.signed(count)?,
                bits.signed(count)?,
            ])
        };
        let mult = if has_mult {
            Some(read4(&mut bits)?)
        } else {
            None
        };
        let add = if has_add {
            Some(read4(&mut bits)?)
        } else {
            None
        };
        Some((Self { mult, add }, bits.byte_pos()))
    }

    fn write(&self, out: &mut Vec<u8>) -> Option<()> {
        let mut values = Vec::new();
        values.extend(self.mult.iter().flatten());
        values.extend(self.add.iter().flatten());
        let count = signed_bits(&values).max(1);
        if count > 15 {
            return None;
        }
        let mut bits = BitWriter::default();
        bits.put(u32::from(self.add.is_some()), 1);
        bits.put(u32::from(self.mult.is_some()), 1);
        bits.put(count, 4);
        for terms in [self.mult, self.add].into_iter().flatten() {
            for value in terms {
                bits.put_signed(value, count);
            }
        }
        out.extend_from_slice(&bits.out);
        Some(())
    }

    /// The same transform with the alpha multiplier at zero.
    fn transparent(current: Option<Self>) -> Self {
        let mut result = current.unwrap_or(Self {
            mult: None,
            add: None,
        });
        let mut mult = result.mult.unwrap_or([256, 256, 256, 256]);
        mult[3] = 0;
        result.mult = Some(mult);
        result
    }

    /// The same transform at half opacity.
    fn dimmed(current: Option<Self>) -> Self {
        let mut result = Self::transparent(current);
        if let Some(mult) = result.mult.as_mut() {
            mult[3] = DIMMED_ALPHA;
        }
        result
    }
}

/// PlaceObject2 with every optional field decoded; clip actions and anything
/// after them are kept as raw bytes.
#[derive(Clone, Debug, PartialEq)]
struct PlaceObject2 {
    flags: u8,
    depth: u16,
    character: Option<u16>,
    matrix: Option<Matrix>,
    color: Option<ColorTransform>,
    ratio: Option<u16>,
    name: Option<Vec<u8>>,
    clip_depth: Option<u16>,
    rest: Vec<u8>,
}

impl PlaceObject2 {
    fn parse(body: &[u8]) -> Option<Self> {
        let flags = *body.first()?;
        let depth = read_u16(body, 1)?;
        let mut pos = 3;
        let character = if flags & 0x02 != 0 {
            let value = read_u16(body, pos)?;
            pos += 2;
            Some(value)
        } else {
            None
        };
        let matrix = if flags & 0x04 != 0 {
            let (value, next) = Matrix::read(body, pos)?;
            pos = next;
            Some(value)
        } else {
            None
        };
        let color = if flags & 0x08 != 0 {
            let (value, next) = ColorTransform::read(body, pos)?;
            pos = next;
            Some(value)
        } else {
            None
        };
        let ratio = if flags & 0x10 != 0 {
            let value = read_u16(body, pos)?;
            pos += 2;
            Some(value)
        } else {
            None
        };
        let name = if flags & 0x20 != 0 {
            let end = pos + body.get(pos..)?.iter().position(|&byte| byte == 0)?;
            let value = body[pos..end].to_vec();
            pos = end + 1;
            Some(value)
        } else {
            None
        };
        let clip_depth = if flags & 0x40 != 0 {
            let value = read_u16(body, pos)?;
            pos += 2;
            Some(value)
        } else {
            None
        };
        let rest = body.get(pos..)?.to_vec();
        if flags & 0x80 == 0 && !rest.is_empty() {
            return None;
        }
        Some(Self {
            flags,
            depth,
            character,
            matrix,
            color,
            ratio,
            name,
            clip_depth,
            rest,
        })
    }

    fn encode(&self) -> Option<Vec<u8>> {
        let mut flags = self.flags & 0x81;
        flags |= if self.character.is_some() { 0x02 } else { 0 };
        flags |= if self.matrix.is_some() { 0x04 } else { 0 };
        flags |= if self.color.is_some() { 0x08 } else { 0 };
        flags |= if self.ratio.is_some() { 0x10 } else { 0 };
        flags |= if self.name.is_some() { 0x20 } else { 0 };
        flags |= if self.clip_depth.is_some() { 0x40 } else { 0 };
        let mut out = vec![flags];
        out.extend_from_slice(&self.depth.to_le_bytes());
        if let Some(character) = self.character {
            out.extend_from_slice(&character.to_le_bytes());
        }
        if let Some(matrix) = &self.matrix {
            matrix.write(&mut out)?;
        }
        if let Some(color) = &self.color {
            color.write(&mut out)?;
        }
        if let Some(ratio) = self.ratio {
            out.extend_from_slice(&ratio.to_le_bytes());
        }
        if let Some(name) = &self.name {
            out.extend_from_slice(name);
            out.push(0);
        }
        if let Some(clip_depth) = self.clip_depth {
            out.extend_from_slice(&clip_depth.to_le_bytes());
        }
        out.extend_from_slice(&self.rest);
        Some(out)
    }

    fn named(&self, name: &[u8]) -> bool {
        self.name.as_deref() == Some(name)
    }
}

fn find_placement(tags: &[Tag], name: &[u8]) -> Option<(usize, PlaceObject2)> {
    let mut found = None;
    for (index, tag) in tags.iter().enumerate() {
        if tag.code != TAG_PLACE_OBJECT2 {
            continue;
        }
        let Some(place) = PlaceObject2::parse(&tag.body) else {
            continue;
        };
        if place.named(name) {
            if found.is_some() {
                return None; // ambiguous: leave it alone
            }
            found = Some((index, place));
        }
    }
    found
}

fn replace_placement(tags: &mut [Tag], index: usize, place: &PlaceObject2) -> Option<()> {
    tags[index].body = place.encode()?;
    Some(())
}

/// What `patch_hud_movie` changed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct PatchReport {
    pub buddy_panel_hidden: bool,
    pub ash_rows: bool,
    /// The row's dimmed name line (for an Ash being dismissed) is in place.
    pub dimmed_names: bool,
    /// RemotePCTag6 was added for a seventh row.
    pub extra_row: bool,
    pub changed: bool,
}

/// Patches a loaded `01_000_fe.gfx`. Returns the new movie when anything
/// changed, `Ok(None)` when it already has every change, and an error when
/// the movie cannot be read safely (then it must be served unchanged).
pub(crate) fn patch_hud_movie(input: &[u8]) -> Result<(Option<Vec<u8>>, PatchReport), String> {
    let signature = input.get(0..3).ok_or("movie too short")?;
    if signature != b"GFX" && signature != b"FWS" {
        return Err("not an uncompressed GFX/SWF movie".into());
    }
    let declared = read_u32(input, 4).ok_or("movie too short")? as usize;
    // The game's buffer may carry archive padding after the declared length.
    if declared < 16 || declared > input.len() {
        return Err(format!(
            "declared length {declared} does not fit the {} loaded bytes",
            input.len()
        ));
    }
    let data = &input[..declared];
    if fnv1a(data) != 8212292726891708480u64 {
        return Err("Unsupported HUD asset fingerprint; original movie preserved".into());
    }
    let rect_bits = u32::from(data[8] >> 3);
    let header_len = 8 + (5 + 4 * rect_bits as usize).div_ceil(8) + 4;
    if header_len >= data.len() {
        return Err("movie header is truncated".into());
    }
    let mut tags = parse_tags(data, header_len).map_err(str::to_owned)?;
    let original = tags.clone();
    let mut report = PatchReport::default();

    // Group 1: hide the single-Ash panel.
    let buddy = find_placement(&tags, BUDDY_PANEL);
    if let Some((index, mut place)) = buddy.clone() {
        place.color = Some(ColorTransform::transparent(place.color));
        if replace_placement(&mut tags, index, &place).is_some() {
            report.buddy_panel_hidden = true;
        }
    }

    // Group 2: turn the remote-player tags into the Ash rows.
    // The seventh tag only goes in with the rows.
    let mut with_row_tags = tags.clone();
    isolate_row_sprite(&mut with_row_tags).ok_or("Cannot isolate four-row sprite")?;
    let added = false; // No extra row placements; only an isolated template.
    if let Some(rows) = plan_ash_rows(&with_row_tags, buddy.as_ref().map(|(_, place)| place)) {
        tags = with_row_tags;
        report.extra_row = added;
        if rows.already_applied {
            report.ash_rows = true;
        } else {
            let mut candidate = tags.clone();
            if apply_ash_rows(&mut candidate, &rows).is_some() {
                tags = candidate;
                report.ash_rows = true;
            }
        }
        // Own step, so movies laid out by an earlier package also get it.
        if report.ash_rows {
            if rows.dimmed_names_applied {
                report.dimmed_names = true;
            } else {
                let mut candidate = tags.clone();
                if apply_dimmed_names(&mut candidate, rows.sprite_index).is_some() {
                    tags = candidate;
                    report.dimmed_names = true;
                }
            }
        }
    }

    if !report.buddy_panel_hidden || !report.ash_rows {
        return Err("Required Buddy/four-row layout incomplete; original movie preserved".into());
    }
    if tags == original {
        return Err("HUD movie was already modified; refusing ambiguous layout".into());
    }
    report.changed = true;
    let mut out = Vec::with_capacity(declared + 256);
    out.extend_from_slice(&data[..header_len]);
    write_tags(&mut out, &tags).map_err(str::to_owned)?;
    let total = u32::try_from(out.len()).map_err(|_| "movie too large")?;
    out[4..8].copy_from_slice(&total.to_le_bytes());
    Ok((Some(out), report))
}

/// Clone only the first four rows' shared sprite. Conservatively reject an
/// occupied reserved character ID anywhere, including unknown tag bodies.
fn isolate_row_sprite(tags: &mut Vec<Tag>) -> Option<()> {
    let id = 65535u16;
    if tags.iter().any(|t| read_u16(&t.body, 0) == Some(id)) {
        return None;
    }
    let (_, first) = find_placement(tags, b"RemotePCTag0")?;
    let original = first.character?;
    for row in 1..ROW_COUNT {
        let name = format!("RemotePCTag{row}");
        let (_, place) = find_placement(tags, name.as_bytes())?;
        if place.character != Some(original) {
            return None;
        }
    }
    let at = tags
        .iter()
        .position(|t| t.code == TAG_DEFINE_SPRITE && read_u16(&t.body, 0) == Some(original))?;
    let mut clone = tags[at].clone();
    clone.body[..2].copy_from_slice(&id.to_le_bytes());
    tags.insert(at + 1, clone);
    for row in 0..ROW_COUNT {
        let name = format!("RemotePCTag{row}");
        let (index, mut place) = find_placement(tags, name.as_bytes())?;
        place.character = Some(id);
        replace_placement(tags, index, &place)?;
    }
    Some(())
}

struct AshRowsPlan {
    rows: Vec<(usize, PlaceObject2)>,
    sprite_index: usize,
    base_y: i32,
    /// Vertical scale of the Buddy panel (16.16); another HUD layout may
    /// have scaled the HUD, and the row spacing follows it.
    base_scale: i32,
    /// Every hidden icon is already transparent: the rows were laid out by
    /// an earlier pass (or a movie that already carries this layout).
    already_applied: bool,
    /// The role line already sits on the name line at half opacity.
    dimmed_names_applied: bool,
}

fn plan_ash_rows(tags: &[Tag], buddy: Option<&PlaceObject2>) -> Option<AshRowsPlan> {
    let buddy_matrix = buddy?.matrix?;
    let base_y = buddy_matrix.translate.1;
    let base_scale = buddy_matrix.scale.map_or(FIXED_ONE, |(_, y)| y);
    if buddy_matrix.rotate.is_some() || base_scale <= 0 {
        return None;
    }
    let mut rows = Vec::with_capacity(ROW_COUNT);
    let mut character = None;
    for row in 0..ROW_COUNT {
        let mut name = ROW_TAG_PREFIX.to_vec();
        name.extend_from_slice(row.to_string().as_bytes());
        let (index, place) = find_placement(tags, &name)?;
        let matrix = place.matrix?;
        if matrix.rotate.is_some() || place.character.is_none() {
            return None;
        }
        if character.is_some() && character != place.character {
            return None;
        }
        character = place.character;
        rows.push((index, place));
    }
    let character = character?;
    let sprite_index = tags.iter().position(|tag| {
        tag.code == TAG_DEFINE_SPRITE && read_u16(&tag.body, 0) == Some(character)
    })?;
    let children = parse_tags(&tags[sprite_index].body, 4).ok()?;
    let mut already_applied = true;
    for icon in HIDDEN_ROW_ICONS {
        let (_, place) = find_placement(&children, icon)?;
        already_applied &= place.color.and_then(|color| color.mult).map(|mult| mult[3]) == Some(0);
    }
    let (_, name) = find_placement(&children, ROW_NAME)?;
    let (_, dimmed) = find_placement(&children, DIMMED_ROW_NAME)?;
    let dimmed_names_applied = dimmed
        .color
        .and_then(|color| color.mult)
        .map(|mult| mult[3])
        == Some(DIMMED_ALPHA)
        && dimmed.matrix.map(|matrix| matrix.translate)
            == name.matrix.map(|matrix| matrix.translate);
    Some(AshRowsPlan {
        rows,
        sprite_index,
        base_y,
        base_scale,
        already_applied,
        dimmed_names_applied,
    })
}

const FIXED_ONE: i32 = 0x1_0000;

/// value * factor for a 16.16 fixed-point factor, rounded to nearest.
fn scale_fixed(value: i32, factor: i32) -> Option<i32> {
    let product = i64::from(value) * i64::from(factor);
    i32::try_from((product + product.signum() * 0x8000) / 0x1_0000).ok()
}

fn apply_ash_rows(tags: &mut [Tag], plan: &AshRowsPlan) -> Option<()> {
    // Hide the unused icons inside the row template sprite.
    let sprite = &tags[plan.sprite_index].body;
    let header = sprite.get(0..4)?.to_vec();
    let mut children = parse_tags(sprite, 4).ok()?;
    for icon in HIDDEN_ROW_ICONS {
        let (index, mut place) = find_placement(&children, icon)?;
        place.color = Some(ColorTransform::transparent(place.color));
        replace_placement(&mut children, index, &place)?;
    }
    let mut body = header;
    write_tags(&mut body, &children).ok()?;
    tags[plan.sprite_index].body = body;

    // Stack the rows from the Buddy panel, scaled down. Both are relative to
    // what the loaded movie has, so a HUD scale applied by another layout
    // is kept (vanilla: scale 1 -> 0.82, 45 px spacing).
    let spacing = scale_fixed(
        (crate::runtime::config().hud_spacing * 20.0).round() as i32,
        plan.base_scale,
    )?;
    let shift_x = scale_fixed(
        (crate::runtime::config().hud_x * 20.0).round() as i32,
        plan.base_scale,
    )?;
    let base_y = plan.base_y.checked_add(scale_fixed(
        (crate::runtime::config().hud_y * 20.0).round() as i32,
        plan.base_scale,
    )?)?;
    for (row, (index, place)) in plan.rows.iter().enumerate() {
        let mut place = place.clone();
        let mut matrix = place.matrix?;
        let (scale_x, scale_y) = matrix.scale.unwrap_or((FIXED_ONE, FIXED_ONE));
        matrix.scale = Some((
            scale_fixed(
                scale_x,
                (crate::runtime::config().hud_scale * 65536.0).round() as i32,
            )?,
            scale_fixed(
                scale_y,
                (crate::runtime::config().hud_scale * 65536.0).round() as i32,
            )?,
        ));
        matrix.translate.1 = base_y.checked_add(spacing.checked_mul(row as i32)?)?;
        matrix.translate.0 = matrix.translate.0.checked_add(shift_x)?;
        place.matrix = Some(matrix);
        replace_placement(tags, *index, &place)?;
    }
    Some(())
}

/// Puts the row's role line (`RoleName`, the same text clip as `Name`) on the
/// name line at half opacity. The HUD writes a dismissed Ash's name there and
/// leaves `Name` empty, so the row reads dimmed; other rows leave it empty.
fn apply_dimmed_names(tags: &mut [Tag], sprite_index: usize) -> Option<()> {
    let sprite = &tags[sprite_index].body;
    let header = sprite.get(0..4)?.to_vec();
    let mut children = parse_tags(sprite, 4).ok()?;
    let (_, name) = find_placement(&children, ROW_NAME)?;
    let (index, mut dimmed) = find_placement(&children, DIMMED_ROW_NAME)?;
    let mut matrix = dimmed.matrix?;
    matrix.translate = name.matrix?.translate;
    dimmed.matrix = Some(matrix);
    dimmed.color = Some(ColorTransform::dimmed(dimmed.color));
    replace_placement(&mut children, index, &dimmed)?;
    let mut body = header;
    write_tags(&mut body, &children).ok()?;
    tags[sprite_index].body = body;
    Some(())
}

// ---------------------------------------------------------------------------
// Locating the game's Scaleform file types (works on a mapped image or on a
// file laid out as an image, so it can be checked offline)
// ---------------------------------------------------------------------------

pub(crate) struct Section<'a> {
    pub name: [u8; 8],
    pub rva: u32,
    pub data: &'a [u8],
    pub executable: bool,
}

pub(crate) struct ImageView<'a> {
    pub base: u64,
    pub sections: Vec<Section<'a>>,
}

impl ImageView<'_> {
    fn section_named(&self, name: &[u8]) -> Option<&Section<'_>> {
        self.sections.iter().find(|section| {
            section.name.starts_with(name) && section.name.get(name.len()) == Some(&0)
        })
    }

    fn read(&self, rva: u32, len: usize) -> Option<&[u8]> {
        self.sections.iter().find_map(|section| {
            let offset = rva.checked_sub(section.rva)? as usize;
            section.data.get(offset..offset.checked_add(len)?)
        })
    }

    fn read_u64(&self, rva: u32) -> Option<u64> {
        Some(u64::from_le_bytes(self.read(rva, 8)?.try_into().ok()?))
    }

    fn is_code(&self, rva: u32) -> bool {
        self.sections.iter().any(|section| {
            section.executable
                && rva >= section.rva
                && ((rva - section.rva) as usize) < section.data.len()
        })
    }

    fn rva_of_va(&self, va: u64) -> Option<u32> {
        u32::try_from(va.checked_sub(self.base)?).ok()
    }

    /// Primary vtable of a class by its MSVC RTTI decorated name.
    pub(crate) fn rtti_vtable(&self, decorated: &[u8]) -> Option<u32> {
        let mut descriptor = None;
        'sections: for section in self.sections.iter().filter(|section| !section.executable) {
            let data = section.data;
            let mut offset = 16;
            while offset + decorated.len() < data.len() {
                if data[offset] == decorated[0]
                    && &data[offset..offset + decorated.len()] == decorated
                    && data[offset + decorated.len()] == 0
                {
                    descriptor = Some(section.rva + offset as u32 - 16);
                    break 'sections;
                }
                offset += 1;
            }
        }
        let descriptor = descriptor?;
        let rdata = self.section_named(b".rdata")?;
        let read32 = |offset: usize| -> Option<u32> {
            Some(u32::from_le_bytes(
                rdata.data.get(offset..offset + 4)?.try_into().ok()?,
            ))
        };
        let mut locator = None;
        let mut offset = 0;
        while offset + 24 <= rdata.data.len() {
            let rva = rdata.rva + offset as u32;
            if read32(offset) == Some(1)
                && read32(offset + 4) == Some(0)
                && read32(offset + 12) == Some(descriptor)
                && read32(offset + 20) == Some(rva)
            {
                locator = Some(rva);
                break;
            }
            offset += 4;
        }
        let locator_va = self.base + u64::from(locator?);
        let mut offset = 0;
        while offset + 16 <= rdata.data.len() {
            if rdata.data[offset..offset + 8] == locator_va.to_le_bytes() {
                return Some(rdata.rva + offset as u32 + 8);
            }
            offset += 8;
        }
        None
    }
}

/// `CSScaleformFileOpener::OpenFile` entry: four whole push instructions
/// (push rbp; push rbx; push rsi; push rdi) that the hook relocates, followed
/// by the rest of the verified prologue.
pub(crate) const OPEN_FILE_PROLOGUE: [u8; 16] = [
    0x40, 0x55, 0x53, 0x56, 0x57, 0x41, 0x54, 0x41, 0x56, 0x41, 0x57, 0x48, 0x8d, 0x6c, 0x24, 0xd9,
];
pub(crate) const OPEN_FILE_RELOCATED: usize = 5;
// Scaleform::File slots and the MemoryFile fields their code reads.
const SLOT_IS_VALID: usize = 2;
const SLOT_TELL: usize = 4;
const SLOT_GET_LENGTH: usize = 6;
const IS_VALID_CODE: [u8; 5] = [0x0f, 0xb6, 0x41, 0x28, 0xc3]; // movzx eax, byte [rcx+0x28]
const TELL_CODE: [u8; 4] = [0x8b, 0x41, 0x24, 0xc3]; // mov eax, [rcx+0x24]
const GET_LENGTH_CODE: [u8; 4] = [0x8b, 0x41, 0x20, 0xc3]; // mov eax, [rcx+0x20]
const MEMORY_FILE_BUFFER: usize = 0x18;
const MEMORY_FILE_SIZE: usize = 0x20;
const MEMORY_FILE_INDEX: usize = 0x24;
const MEMORY_FILE_VALID: usize = 0x28;
const MAX_MOVIE_BYTES: usize = 16 * 1024 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ScaleformTargets {
    pub open_file: u32,
    pub memory_file_vtable: u32,
    /// The entry already starts with a rel32 jump (another hook).
    pub open_file_hooked: bool,
}

pub(crate) fn locate_scaleform_targets(
    image: &ImageView<'_>,
) -> Result<ScaleformTargets, &'static str> {
    let opener = image
        .rtti_vtable(b".?AVCSScaleformFileOpener@CS@@")
        .ok_or("CSScaleformFileOpener RTTI not found")?;
    let memory_file = image
        .rtti_vtable(b".?AVMemoryFile@Scaleform@@")
        .ok_or("Scaleform::MemoryFile RTTI not found")?;
    let slot = |vtable: u32, index: usize| -> Option<u32> {
        image.rva_of_va(image.read_u64(vtable + 8 * index as u32)?)
    };
    for (index, code) in [
        (SLOT_IS_VALID, &IS_VALID_CODE[..]),
        (SLOT_TELL, &TELL_CODE[..]),
        (SLOT_GET_LENGTH, &GET_LENGTH_CODE[..]),
    ] {
        let target = slot(memory_file, index).ok_or("MemoryFile slot outside the executable")?;
        if !image.is_code(target) || image.read(target, code.len()) != Some(code) {
            return Err("MemoryFile field layout differs from the verified one");
        }
    }
    let open_file = slot(opener, 1).ok_or("OpenFile slot outside the executable")?;
    if !image.is_code(open_file) {
        return Err("OpenFile is not code");
    }
    let entry = image
        .read(open_file, OPEN_FILE_PROLOGUE.len())
        .ok_or("OpenFile unreadable")?;
    let open_file_hooked = entry[0] == 0xe9;
    if !open_file_hooked && entry != OPEN_FILE_PROLOGUE {
        return Err("OpenFile prologue differs from the verified one");
    }
    if open_file % 8 != 0 {
        return Err("OpenFile entry is not 8-byte aligned");
    }
    Ok(ScaleformTargets {
        open_file,
        memory_file_vtable: memory_file,
        open_file_hooked,
    })
}

// ---------------------------------------------------------------------------
// Runtime hook
// ---------------------------------------------------------------------------

type OpenFileFn = unsafe extern "system" fn(*mut c_void, *const u8, i32, i32) -> *mut c_void;

static INSTALLED: AtomicBool = AtomicBool::new(false);
static ORIGINAL_OPEN_FILE: AtomicUsize = AtomicUsize::new(0);
static MEMORY_FILE_VTABLE: AtomicUsize = AtomicUsize::new(0);
/// The loaded HUD movie has the dimmed name line (see `apply_dimmed_names`).
pub(crate) static DIMMED_NAMES_READY: AtomicBool = AtomicBool::new(false);
/// Patched movies by input hash; served buffers stay alive for the process.
static PATCHED: LazyLock<Mutex<HashMap<(u64, usize), Option<&'static [u8]>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn fnv1a(data: &[u8]) -> u64 {
    data.iter().fold(0xcbf2_9ce4_8422_2325u64, |hash, &byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

fn url_is_hud_movie(url: *const u8) -> bool {
    if url.is_null() {
        return false;
    }
    let bytes = unsafe { std::ffi::CStr::from_ptr(url.cast()) }.to_bytes();
    let name = HUD_MOVIE_FILE.as_bytes();
    bytes.len() >= name.len()
        && bytes[bytes.len() - name.len()..].eq_ignore_ascii_case(name)
        && (bytes.len() == name.len()
            || matches!(bytes[bytes.len() - name.len() - 1], b'/' | b'\\' | b':'))
}

unsafe extern "system" fn open_file_detour(
    opener: *mut c_void,
    url: *const u8,
    flags: i32,
    mode: i32,
) -> *mut c_void {
    let original: OpenFileFn =
        unsafe { std::mem::transmute(ORIGINAL_OPEN_FILE.load(Ordering::Acquire)) };
    let file = unsafe { original(opener, url, flags, mode) };
    if file.is_null() || !crate::hud::INSTALLED.load(Ordering::Acquire) || !url_is_hud_movie(url) {
        return file;
    }
    crate::hud::MOVIE_READY.store(false, Ordering::Release);
    let _ = std::panic::catch_unwind(|| unsafe { patch_open_movie(file) });
    file
}

unsafe fn read_field<T: Copy>(object: *mut c_void, offset: usize) -> T {
    unsafe { object.cast::<u8>().add(offset).cast::<T>().read_unaligned() }
}

unsafe fn write_field<T: Copy>(object: *mut c_void, offset: usize, value: T) {
    unsafe {
        object
            .cast::<u8>()
            .add(offset)
            .cast::<T>()
            .write_unaligned(value)
    }
}

/// Swaps the movie bytes of a MemoryFile (or of a wrapper that reuses the
/// MemoryFile accessors, i.e. the same field layout). Any other file type is
/// left alone.
unsafe fn patch_open_movie(file: *mut c_void) {
    let memory_vtable = MEMORY_FILE_VTABLE.load(Ordering::Acquire) as *const usize;
    let vtable = unsafe { read_field::<*const usize>(file, 0) };
    let same_layout = vtable == memory_vtable
        || [SLOT_IS_VALID, SLOT_TELL, SLOT_GET_LENGTH]
            .iter()
            .all(|&slot| unsafe { *vtable.add(slot) == *memory_vtable.add(slot) });
    if !same_layout {
        crate::runtime::log("HUD movie: opened through an unknown file type; served unchanged");
        return;
    }
    if unsafe { read_field::<u8>(file, MEMORY_FILE_VALID) } == 0 {
        return;
    }
    let buffer = unsafe { read_field::<*const u8>(file, MEMORY_FILE_BUFFER) };
    let size = unsafe { read_field::<i32>(file, MEMORY_FILE_SIZE) };
    if buffer.is_null() || size <= 0 || size as usize > MAX_MOVIE_BYTES {
        return;
    }
    let input = unsafe { std::slice::from_raw_parts(buffer, size as usize) };
    let key = (fnv1a(input), input.len());
    let Ok(mut cache) = PATCHED.lock() else {
        return;
    };
    if cache.len() >= 4 && !cache.contains_key(&key) {
        return;
    }
    let output = *cache.entry(key).or_insert_with(|| match patch_hud_movie(input) {
        Ok((Some(bytes), report)) => {
            crate::runtime::log(format_args!(
                "HUD movie patched in memory ({} -> {} bytes): buddy_panel_hidden={} ash_rows={} dimmed_names={}",
                input.len(),
                bytes.len(),
                report.buddy_panel_hidden,
                report.ash_rows,
                report.dimmed_names
            ));
            DIMMED_NAMES_READY.store(report.dimmed_names, Ordering::Release);
            Some(Box::leak(bytes.into_boxed_slice()))
        }
        Ok((None, report)) => {
            crate::runtime::log(format_args!(
                "HUD movie already has the changes it can take: buddy_panel_hidden={} ash_rows={} dimmed_names={}",
                report.buddy_panel_hidden, report.ash_rows, report.dimmed_names
            ));
            DIMMED_NAMES_READY.store(report.dimmed_names, Ordering::Release);
            None
        }
        Err(reason) => {
            crate::runtime::log(format_args!("HUD movie served unchanged: {reason}"));
            None
        }
    });
    if let Some(bytes) = output {
        crate::hud::MOVIE_READY.store(true, Ordering::Release);
        unsafe {
            write_field(file, MEMORY_FILE_BUFFER, bytes.as_ptr());
            write_field(file, MEMORY_FILE_SIZE, bytes.len() as i32);
            write_field(file, MEMORY_FILE_INDEX, 0i32);
        }
    }
}

/// Builds an [`ImageView`] over the running executable's readable sections.
pub(crate) fn running_image() -> Option<ImageView<'static>> {
    use windows::Win32::System::{
        LibraryLoader::GetModuleHandleW,
        Memory::{MEM_COMMIT, MEMORY_BASIC_INFORMATION, PAGE_GUARD, PAGE_NOACCESS, VirtualQuery},
    };
    let base = unsafe { GetModuleHandleW(None) }.ok()?.0 as usize;
    let readable = |address: usize, len: usize| -> bool {
        let mut cursor = address;
        while cursor < address + len {
            let mut info = MEMORY_BASIC_INFORMATION::default();
            let written = unsafe {
                VirtualQuery(
                    Some(cursor as *const c_void),
                    &mut info,
                    size_of::<MEMORY_BASIC_INFORMATION>(),
                )
            };
            if written == 0
                || info.State != MEM_COMMIT
                || info.Protect.0 & (PAGE_NOACCESS.0 | PAGE_GUARD.0) != 0
            {
                return false;
            }
            cursor = info.BaseAddress as usize + info.RegionSize;
        }
        true
    };
    if !readable(base, 0x1000) {
        return None;
    }
    let headers = unsafe { std::slice::from_raw_parts(base as *const u8, 0x1000) };
    let pe = read_u32(headers, 0x3c)? as usize;
    let count = usize::from(read_u16(headers, pe + 6)?);
    let optional = usize::from(read_u16(headers, pe + 20)?);
    let mut sections = Vec::new();
    for index in 0..count {
        let entry = pe + 24 + optional + index * 40;
        let name: [u8; 8] = headers.get(entry..entry + 8)?.try_into().ok()?;
        let virtual_size = read_u32(headers, entry + 8)? as usize;
        let rva = read_u32(headers, entry + 12)?;
        let characteristics = read_u32(headers, entry + 36)?;
        let start = base + rva as usize;
        if virtual_size == 0 || !readable(start, virtual_size) {
            continue;
        }
        sections.push(Section {
            name,
            rva,
            data: unsafe { std::slice::from_raw_parts(start as *const u8, virtual_size) },
            executable: characteristics & 0x2000_0000 != 0,
        });
    }
    Some(ImageView {
        base: base as u64,
        sections,
    })
}

/// Installs the OpenFile hook. Call as early as possible: a HUD movie opened
/// before this is served unchanged. Returns whether the hook is active.
pub(crate) fn prepare() -> Result<(usize, Vec<u8>, Vec<u8>), String> {
    let image = running_image().ok_or("HUD executable image unavailable")?;
    let targets = locate_scaleform_targets(&image).map_err(str::to_owned)?;
    if targets.open_file_hooked {
        return Err("HUD OpenFile already hooked; refusing chain".into());
    }
    let entry = image.base as usize + targets.open_file as usize;
    let page = unsafe { crate::hooks::near(entry) }.ok_or("HUD relay allocation")?;
    let old = OPEN_FILE_PROLOGUE[..5].to_vec();
    let mut relay = vec![0xff, 0x25, 0, 0, 0, 0];
    relay.extend_from_slice(&(open_file_detour as *const () as usize).to_le_bytes());
    let mut trampoline = old.clone();
    trampoline.extend_from_slice(&[0xff, 0x25, 0, 0, 0, 0]);
    trampoline.extend_from_slice(&(entry + 5).to_le_bytes());
    unsafe {
        std::ptr::copy_nonoverlapping(relay.as_ptr(), page as *mut u8, relay.len());
        std::ptr::copy_nonoverlapping(
            trampoline.as_ptr(),
            (page + 32) as *mut u8,
            trampoline.len(),
        );
        crate::hooks::seal_page(page)?;
    }
    ORIGINAL_OPEN_FILE.store(page + 32, Ordering::Release);
    MEMORY_FILE_VTABLE.store(
        image.base as usize + targets.memory_file_vtable as usize,
        Ordering::Release,
    );
    let new = crate::hooks::call_bytes(entry, page).ok_or("HUD relay range")?;
    let mut new = new.to_vec();
    new[0] = 0xe9;
    Ok((entry, old, new))
}
