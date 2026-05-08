-- ok
select U&"d\0061t\+000061";
select U&"\\";
select U&"ok: !0061" UESCAPE '!';
select U&" \" UESCAPE '!';

-- errors
select U&"\006";
select U&"\+0061";
select U&"wrong: \06" UESCAPE '\';
select U&"wrong: !061" UESCAPE '!';
select U&" \";
