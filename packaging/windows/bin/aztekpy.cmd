@echo off
rem aztekpy from a command prompt: the program in the install folder above this one, with what was typed after it.
"%~dp0..\aztekpy.exe" %*
exit /b %ERRORLEVEL%
