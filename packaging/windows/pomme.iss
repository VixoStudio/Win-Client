; Installs the Win Client bundle (win-client.exe and OpenAL notices) for the
; current user, into a folder the client's updater can write to.
; This downstream AppId is intentionally separate from the upstream Pomme installer.
; The legacy data migration is handled by the client, not by Inno Setup, so changing
; this identifier must not be treated as a data-migration mechanism.
;
; usage: iscc /DAppVersion=<x.y.z> /DBundle=<bundle-dir> /DOutDir=<out-dir> /DOutName=<name> pomme.iss

[Setup]
AppId={{B7E3A4D1-5C2F-4B9E-8A61-3D0F6C2A9174}
AppName=Win Client
AppVersion={#AppVersion}
AppPublisher=WinModing
AppPublisherURL=https://github.com/vithorixscom-debug/Win-Client
DefaultDirName={localappdata}\Programs\Win Client
DisableProgramGroupPage=yes
PrivilegesRequired=lowest
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
OutputDir={#OutDir}
OutputBaseFilename={#OutName}
SetupIconFile=..\..\pomme-client\assets\icon.ico
UninstallDisplayIcon={app}\win-client.exe
WizardStyle=modern
Compression=lzma2
SolidCompression=yes

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
Source: "{#Bundle}\*"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{autoprograms}\Win Client"; Filename: "{app}\win-client.exe"
Name: "{autodesktop}\Win Client"; Filename: "{app}\win-client.exe"; Tasks: desktopicon

[Run]
Filename: "{app}\win-client.exe"; Description: "{cm:LaunchProgram,Win Client}"; Flags: nowait postinstall skipifsilent

; What the updater leaves behind; the data folder (%LOCALAPPDATA%\Win Client) stays.
[UninstallDelete]
Type: files; Name: "{app}\*.old"
Type: filesandordirs; Name: "{app}\.pomme-update"
