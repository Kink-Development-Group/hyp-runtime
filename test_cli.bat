@echo off
echo Testing CLI...
dotnet run --project HypnoScript.CLI run test_simple.hyp --debug
echo CLI Test finished.
pause
