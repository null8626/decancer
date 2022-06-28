@echo off
setlocal
node.exe "%cd%\node_modules\@napi-rs\cli\scripts\index.js" %*
endlocal