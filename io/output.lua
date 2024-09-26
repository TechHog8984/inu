-- disassembled by inu in 81 microseconds

0    OpGetGlobal(ABX(0, 0))    -- r_0 = b
1    OpMove(ABC(1, 0, 0))    -- r_1 = r_0
2    OpLoadK(ABX(2, 1))    -- r_2 = "\a, \b, \c, \n, \r, \t, \v, \\, \', \", Hello, World!"
3    OpLoadBool(ABC(1, 1, 0))    -- r_1 = true
4    OpLoadNil(ABC(3, 3, 0))    -- r_3..r_3 = nil
5    OpNewTable(ABC(4, 1, 0))    -- r_4 = {} -- 1 list & 0 record
6    OpLoadK(ABX(5, 2))    -- r_5 = "foo"
7    OpUnknown(34)    -- TODO: DESCRIBE OpUnknown(34)
8    OpGetTable(ABC(5, 4, 259))    -- r_5 = r_4[1]
9    OpGetTable(ABC(5, 4, 258))    -- r_5 = r_4["foo"]
10    OpGetTable(ABC(5, 4, 1))    -- r_5 = r_4[r_1]
11    OpSetTable(ABC(4, 258, 259))    -- r_4["foo"] = 1
12    OpGetTable(ABC(6, 4, 258))    -- r_6 = r_4["foo"]
13    OpGetTable(ABC(6, 4, 6))    -- r_6 = r_4[r_6]
14    OpSetTable(ABC(4, 260, 6))    -- r_4["bar"] = r_6
15    OpLoadK(ABX(6, 4))    -- r_6 = "bar"
16    OpSetGlobal(ABX(6, 2))    -- foo = r_6
local function proto_0()
    0    OpLoadK(ABX(1, 0))    -- r_1 = 1
    1    OpSetUpval(ABC(1, 0, 0))    -- upvalue_0 = r_1
    2    OpGetUpval(ABC(1, 1, 0))    -- r_1 = upvalue_1
    3    OpUnknown(30)    -- TODO: DESCRIBE OpUnknown(30)
    4    OpUnknown(30)    -- TODO: DESCRIBE OpUnknown(30)
end
17    OpClosure(ABX(6, 0))    -- TODO: DESCRIBE OpClosure(ABX(6, 0))
18    OpMove(ABC(0, 3, 0))    -- r_0 = r_3
19    OpMove(ABC(0, 1, 0))    -- r_0 = r_1
20    OpSetTable(ABC(5, 258, 6))    -- r_5["foo"] = r_6
21    OpSelf(ABC(6, 5, 258))    -- r_7 = r_5; r_6 = r_5["foo"]
22    OpUnknown(28)    -- TODO: DESCRIBE OpUnknown(28)
local function proto_1()
    0    OpUnknown(30)    -- TODO: DESCRIBE OpUnknown(30)
end
23    OpClosure(ABX(6, 1))    -- TODO: DESCRIBE OpClosure(ABX(6, 1))
24    OpAdd(ABC(7, 0, 259))    -- r_7 = r_0 + 1
25    OpAdd(ABC(7, 259, 7))    -- r_7 = 1 + r_7
26    OpSub(ABC(8, 7, 259))    -- r_8 = r_7 - 1
27    OpSub(ABC(8, 259, 8))    -- r_8 = 1 - r_8
28    OpMul(ABC(9, 8, 259))    -- r_9 = r_8 * 1
29    OpMul(ABC(9, 259, 9))    -- r_9 = 1 * r_9
30    OpDiv(ABC(10, 9, 259))    -- r_10 = r_9 / 1
31    OpDiv(ABC(10, 259, 10))    -- r_10 = 1 / r_10
32    OpMod(ABC(11, 10, 259))    -- r_11 = r_10 % 1
33    OpMod(ABC(11, 259, 11))    -- r_11 = 1 % r_11
34    OpPow(ABC(12, 11, 259))    -- r_12 = r_11 ^ 1
35    OpPow(ABC(12, 259, 12))    -- r_12 = 1 ^ r_12
36    OpUnm(ABC(13, 12, 0))    -- r_13 = -r_12
37    OpNot(ABC(14, 13, 0))    -- r_14 = not r_13
38    OpLen(ABC(15, 5, 0))    -- r_15 = #r_5
39    OpLoadK(ABX(16, 5))    -- r_16 = "hello"
40    OpLoadK(ABX(17, 6))    -- r_17 = " "
41    OpLoadK(ABX(18, 7))    -- r_18 = "world"
42    OpConcat(ABC(16, 16, 18))    -- r_16 = r_16 .. ... .. r_18
43    OpUnknown(26)    -- TODO: DESCRIBE OpUnknown(26)
44    OpJmp(ASBX(0, 3))    -- goto 48
45    OpGetGlobal(ABX(17, 8))    -- r_17 = print
46    OpLoadK(ABX(18, 9))    -- r_18 = "hi"
47    OpUnknown(28)    -- TODO: DESCRIBE OpUnknown(28)
48    OpEq(ABC(0, 16, 2))    -- if r_16 == r_2 then goto 50
49    OpJmp(ASBX(0, 3))    -- goto 53
50    OpGetGlobal(ABX(17, 8))    -- r_17 = print
51    OpLoadK(ABX(18, 9))    -- r_18 = "hi"
52    OpUnknown(28)    -- TODO: DESCRIBE OpUnknown(28)
53    OpLt(ABC(0, 16, 2))    -- if r_16 < r_2 then goto 55
54    OpJmp(ASBX(0, 3))    -- goto 58
55    OpGetGlobal(ABX(17, 8))    -- r_17 = print
56    OpLoadK(ABX(18, 9))    -- r_18 = "hi"
57    OpUnknown(28)    -- TODO: DESCRIBE OpUnknown(28)
58    OpLe(ABC(0, 16, 2))    -- if r_16 <= r_2 then goto 60
59    OpJmp(ASBX(0, 3))    -- goto 63
60    OpGetGlobal(ABX(17, 8))    -- r_17 = print
61    OpLoadK(ABX(18, 9))    -- r_18 = "hi"
62    OpUnknown(28)    -- TODO: DESCRIBE OpUnknown(28)
63    OpUnknown(30)    -- TODO: DESCRIBE OpUnknown(30)
