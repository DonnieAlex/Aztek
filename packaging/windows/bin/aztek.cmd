@echo off
rem aztek from a command prompt: the program in the install folder above this one, with what was typed after it.
rem In batch mode it runs in the prompt and gives its exit code; otherwise the window opens and the prompt comes back.
echo %* | findstr /i /c:"batch" >nul
if %ERRORLEVEL%==0 (
    "%~dp0..\aztek.exe" %*
    exit /b %ERRORLEVEL%
)
start "" "%~dp0..\aztek.exe" %*
