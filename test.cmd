@echo off
set /a count = %1
if [%1]==[] (set /a count = 1)

FOR /L %%i IN (1,1, %count% ) DO (
	python test.py|cargo run
)
