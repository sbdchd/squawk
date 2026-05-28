-- ok
select U&'\0061\+000061';
select U&'\\';
select U&'ok: !0061' UESCAPE '!';
select U&' \' UESCAPE '!';

-- errors
select U&'\006';
select U&'\+0061';
select U&'wrong: \06' UESCAPE '\';
select U&'wrong: !061' UESCAPE '!';
select U&'many: \061 \+0061 \zzzz';
select U&'\D800\\';
select U&'\D800\D801\DC00';
select U&' \';
select U&'error' UESCAPE '';
select U&'error' UESCAPE ' ';
select U&'error' UESCAPE '+';
select U&'error' UESCAPE 'A';
select U&'error' UESCAPE 'é';
select U&'error' UESCAPE 'foo';
