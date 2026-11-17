; Aztek installer, for Inno Setup 6. Built by packaging\windows\build-installer.ps1 from what build.ps1 leaves in
; build\dist. Pass the version with iscc /DAppVersion=0.1.0. Not signed: Windows SmartScreen warns about it.

#ifndef AppVersion
  #define AppVersion "0.0.0"
#endif
#ifndef OutputDir
  #define OutputDir "..\..\dist"
#endif

[Setup]
AppId={{6B1F2E3A-7C54-4D0E-9A21-3F5C8E7D1A42}
AppName=Aztek
AppVersion={#AppVersion}
AppVerName=Aztek {#AppVersion}
AppPublisher=Simon Legrand
AppPublisherURL=https://github.com/srlegrand/AKA-Crest-Aztek
DefaultDirName={autopf}\Aztek
DefaultGroupName=Aztek
DisableProgramGroupPage=yes
LicenseFile=..\..\LICENSE
OutputDir={#OutputDir}
OutputBaseFilename=aztek-{#AppVersion}-setup-x64
SetupIconFile=..\..\app\icons\aztek.ico
UninstallDisplayIcon={app}\aztek.exe
Compression=lzma2/max
SolidCompression=yes
WizardStyle=modern
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
PrivilegesRequired=admin
PrivilegesRequiredOverridesAllowed=dialog
ChangesAssociations=yes
ChangesEnvironment=yes

[Tasks]
Name: "desktopicon"; Description: "Create a desktop shortcut"; GroupDescription: "Shortcuts:"; Flags: unchecked
Name: "associate"; Description: "Open .ma files with Aztek"; GroupDescription: "File associations:"; Flags: unchecked
; aztek, aztekpy and harold from any command prompt: the install's bin folder on PATH, as the reference puts its own
; bin there, the machine's for an install for all users, the user's own otherwise. Taken off again by the uninstaller,
; with the install folder itself, which installs before 17/11/2026 put on PATH (#76).
Name: "addtopath"; Description: "Add Aztek to PATH (aztek, aztekpy, harold from a command prompt)"; GroupDescription: "Command line:"

[Files]
Source: "..\..\build\dist\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs
Source: "..\..\LICENSE"; DestDir: "{app}"; Flags: ignoreversion
; bin: a launcher for each program, which runs it from the install folder with what is typed after it.
Source: "bin\aztek.cmd"; DestDir: "{app}\bin"; Flags: ignoreversion
Source: "bin\aztekpy.cmd"; DestDir: "{app}\bin"; Flags: ignoreversion
Source: "bin\harold.cmd"; DestDir: "{app}\bin"; Flags: ignoreversion

[Icons]
Name: "{autoprograms}\Aztek"; Filename: "{app}\aztek.exe"
Name: "{autodesktop}\Aztek"; Filename: "{app}\aztek.exe"; Tasks: desktopicon

[Registry]
; {app}\aztek.exe also stays in "Open with" for .ma after the association is taken over by something else.
Root: HKA; Subkey: "Software\Classes\.ma"; ValueType: string; ValueName: ""; ValueData: "Aztek.Scene"; Flags: uninsdeletevalue; Tasks: associate
Root: HKA; Subkey: "Software\Classes\.ma\OpenWithProgids"; ValueType: string; ValueName: "Aztek.Scene"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associate
Root: HKA; Subkey: "Software\Classes\Aztek.Scene"; ValueType: string; ValueName: ""; ValueData: "ASCII scene"; Flags: uninsdeletekey; Tasks: associate
Root: HKA; Subkey: "Software\Classes\Aztek.Scene\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\aztek.exe,0"; Tasks: associate
Root: HKA; Subkey: "Software\Classes\Aztek.Scene\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\aztek.exe"" ""%1"""; Tasks: associate

Root: HKLM; Subkey: "SYSTEM\CurrentControlSet\Control\Session Manager\Environment"; ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{app}\bin"; Check: IsAdminInstallMode and NeedsAddPath(ExpandConstant('{app}\bin')); Tasks: addtopath
Root: HKCU; Subkey: "Environment"; ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{app}\bin"; Check: (not IsAdminInstallMode) and NeedsAddPath(ExpandConstant('{app}\bin')); Tasks: addtopath

[Run]
Filename: "{app}\aztek.exe"; Description: "Start Aztek"; Flags: nowait postinstall skipifsilent

[Code]
const
  MachineEnv = 'SYSTEM\CurrentControlSet\Control\Session Manager\Environment';
  UserEnv = 'Environment';

function EnvRoot: Integer;
begin
  if IsAdminInstallMode then Result := HKEY_LOCAL_MACHINE else Result := HKEY_CURRENT_USER;
end;

function EnvKey: String;
begin
  if IsAdminInstallMode then Result := MachineEnv else Result := UserEnv;
end;

{ Whether the folder is not on PATH yet, so an install over an older one doesn't add it twice. }
function NeedsAddPath(Dir: String): Boolean;
var
  Paths: String;
begin
  if not RegQueryStringValue(EnvRoot, EnvKey, 'Path', Paths) then
  begin
    Result := True;
    exit;
  end;
  Result := Pos(';' + Uppercase(Dir) + ';', ';' + Uppercase(Paths) + ';') = 0;
end;

{ Takes a folder off PATH, wherever it is in the list. }
procedure RemoveFromPath(Dir: String);
var
  Paths: String;
  P: Integer;
begin
  if not RegQueryStringValue(EnvRoot, EnvKey, 'Path', Paths) then exit;
  Paths := ';' + Paths + ';';
  P := Pos(';' + Uppercase(Dir) + ';', Uppercase(Paths));
  if P = 0 then exit;
  Delete(Paths, P, Length(Dir) + 1);
  Paths := Copy(Paths, 2, Length(Paths) - 2);
  RegWriteExpandStringValue(EnvRoot, EnvKey, 'Path', Paths);
end;

{ An install over an older one moves PATH from the install folder to its bin. }
procedure CurStepChanged(CurStep: TSetupStep);
begin
  if (CurStep = ssPostInstall) and WizardIsTaskSelected('addtopath') then RemoveFromPath(ExpandConstant('{app}'));
end;

{ The uninstaller takes bin, and the install folder older installs added, off PATH again. }
procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
begin
  if CurUninstallStep <> usPostUninstall then exit;
  RemoveFromPath(ExpandConstant('{app}\bin'));
  RemoveFromPath(ExpandConstant('{app}'));
end;
