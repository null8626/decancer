@echo off
cargo.exe build --release
move /y .\target\release\decancer.dll .
move /y .\target\release\decancer.dll.lib .\decancer.lib
clang-cl.exe /MD /nologo /EHsc test.cpp decancer.lib
test.exe