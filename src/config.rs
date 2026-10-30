#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub hud: bool,
    pub hud_x: f64,
    pub hud_y: f64,
    pub hud_spacing: f64,
    pub hud_scale: f64,
    pub multiple: bool,
    pub anywhere: bool,
    pub resummon: bool,
    pub heal: bool,
    pub percent: f64,
    pub debug: bool,
    pub target_diagnostics: bool,
    pub friendly_lockon: bool,
    pub spirit_mend: bool,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            hud: false,
            hud_x: -91.0,
            hud_y: -350.0,
            hud_spacing: 45.0,
            hud_scale: 0.82,
            multiple: true,
            anywhere: false,
            resummon: true,
            heal: true,
            percent: 25.0,
            debug: false,
            target_diagnostics: false,
            friendly_lockon: false,
            spirit_mend: false,
        }
    }
}
impl Config {
    pub fn parse(text: &str) -> Result<Self, String> {
        let mut c = Self::default();
        let mut section = String::new();
        for raw in text.lines() {
            let line = raw
                .split([';', '#'])
                .next()
                .unwrap_or("")
                .trim()
                .trim_start_matches('\u{feff}');
            if line.is_empty() {
                continue;
            }
            if line.starts_with('[') && line.ends_with(']') {
                section = line[1..line.len() - 1].to_ascii_lowercase();
                continue;
            }
            let (key, value) = line
                .split_once('=')
                .ok_or_else(|| format!("Invalid config line: {line}"))?;
            let key = key.trim().to_ascii_lowercase();
            let value = value.trim();
            let boolean = || match value.to_ascii_lowercase().as_str() {
                "true" => Ok(true),
                "false" => Ok(false),
                _ => Err(format!("Invalid boolean: {key}")),
            };
            match (section.as_str(), key.as_str()) {
                ("spiritash", "allowmultipleashtypes") => c.multiple = boolean()?,
                ("spiritash", "allowresummoning") => c.resummon = boolean()?,
                ("spiritash", "healonreuse") => c.heal = boolean()?,
                ("spiritash", "healpercentage") => {
                    c.percent = value.parse().map_err(|_| "Invalid HealPercentage")?;
                    if !c.percent.is_finite() || c.percent <= 0.0 || c.percent > 100.0 {
                        return Err("HealPercentage must be finite and in (0, 100]".into());
                    }
                }
                ("spiritash", "preservesummoningrestrictions") => c.anywhere = !boolean()?,
                ("spiritash", "requirenormalresourcecost") => {
                    if !boolean()? {
                        return Err(format!("{key}=false is unsupported; safety invariant"));
                    }
                }
                ("hud", "enablegrouphud") => c.hud = boolean()?,
                ("hud", "offsetx" | "offsety" | "rowspacing" | "scale") => {
                    let n: f64 = value.parse().map_err(|_| "Invalid HUD number")?;
                    let (min, max) = match key.as_str() {
                        "scale" => (0.5, 1.5),
                        "rowspacing" => (24.0, 150.0),
                        _ => (-1000.0, 1000.0),
                    };
                    if !n.is_finite() || n < min || n > max {
                        return Err(format!("HUD {key} outside {min}..{max}"));
                    }
                    match key.as_str() {
                        "offsetx" => c.hud_x = n,
                        "offsety" => c.hud_y = n,
                        "rowspacing" => c.hud_spacing = n,
                        _ => c.hud_scale = n,
                    }
                }
                ("targeting", "enablefriendlyspiritlockon") => c.friendly_lockon = boolean()?,
                ("sorcery", "enablespiritmend") => c.spirit_mend = boolean()?,
                ("debug", "enablelogging") => c.debug = boolean()?,
                ("debug", "enabletargetdiagnostics") => c.target_diagnostics = boolean()?,
                _ => return Err(format!("Unknown configuration setting: [{section}] {key}")),
            }
        }
        Ok(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn defaults_work() {
        assert_eq!(Config::parse("").unwrap(), Config::default());
    }
    #[test]
    fn example_matches_defaults() {
        assert_eq!(
            Config::parse(include_str!("../config/SpiritAshResummon.ini")).unwrap(),
            Config::default()
        );
    }
    #[test]
    fn rejects_nonfinite_or_outside_range() {
        for n in ["NaN", "inf", "-1", "0", "100.1"] {
            assert!(Config::parse(&format!("[SpiritAsh]\nHealPercentage={n}")).is_err());
        }
    }
    #[test]
    fn costs_stay_required_and_area_policy_is_optional() {
        assert!(Config::parse("[SpiritAsh]\nRequireNormalResourceCost=false").is_err());
        assert!(
            Config::parse("[SpiritAsh]\nPreserveSummoningRestrictions=false")
                .unwrap()
                .anywhere
        );
    }
    #[test]
    fn malformed_config_fails_closed() {
        assert!(Config::parse("[Debug]\nEnableLogging=maybe").is_err());
    }
}
