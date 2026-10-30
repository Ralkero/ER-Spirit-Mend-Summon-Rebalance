from pathlib import Path
p=Path('work/install-0.4.0.ps1');s=p.read_text(encoding='utf-8-sig')
s=s.replace('0.4.0','0.4.1').replace('0.3.1','0.4.0').replace('106B813818B9C66C7295A6616904D8E7F704A04904F3ADD39FB9BC20B9C665EB','3B92A2B3C7DCD4DAE5C2441F8E266333445BF2A2429E2F469AF69763FE472DF6')
a=s.index('if ([regex]::Matches($ini,');b=s.index('[IO.File]::WriteAllText',a)
s=s[:a]+'''if ($ini -match '(?im)^EnableTargetDiagnostics\\s*=') { throw 'Diagnostic config already present; review it' }
if ([regex]::Matches($ini, '(?m)^EnableLogging\\s*=\\s*true\\s*$').Count -ne 1) { throw 'Expected diagnostic logging enabled' }
$ini = [regex]::Replace($ini, '(?m)^EnableLogging\\s*=\\s*true[^\\S\\r\\n]*', "EnableLogging = true`r`nEnableTargetDiagnostics = true")
'''+s[b:]
s=s.replace('PreserveSummoningRestrictions=false; all other settings retained','EnableTargetDiagnostics=true; all other settings retained')
Path('work/install-0.4.1.ps1').write_text(s,encoding='utf-8-sig')
