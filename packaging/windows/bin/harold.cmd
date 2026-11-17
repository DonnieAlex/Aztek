@echo off
rem harold from a command prompt: the program in the install folder above this one, with what was typed after it.
"%~dp0..\harold.exe" %*
exit /b %ERRORLEVEL%
