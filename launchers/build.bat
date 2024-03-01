@ECHO OFF
CLS

SET "RC=C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0\x86\rc.exe"
SET "RESOURCES=src\resources"

"%RC%" "%RESOURCES%.rc"
cargo rustc --bin areca     -- -C link-args="%RESOURCES%.res"
cargo rustc --bin areca_cl  -- -C link-args="%RESOURCES%.res"

MKDIR .\releases
COPY /v/y .\target\debug\areca.exe    .\releases\areca.exe
COPY /v/y .\target\debug\areca_cl.exe .\releases\areca_cl.exe