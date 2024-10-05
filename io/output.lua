-- disassembled by inu in 506 microseconds

0    OpGetGlobal(ABX(0, 0))    -- r_0 = math
1    OpGetGlobal(ABX(1, 1))    -- r_1 = newproxy
2    OpGetGlobal(ABX(2, 2))    -- r_2 = ipairs
3    OpGetTable(ABC(3, 0, 259))    -- r_3 = r_0["floor"]
4    OpGetTable(ABC(4, 0, 260))    -- r_4 = r_0["pi"]
5    OpLoadNil(ABC(5, 5, 0))    -- r_5..r_5 = nil
6    OpLoadBool(ABC(6, 1, 0))    -- r_6 = true
7    OpLoadBool(ABC(7, 0, 0))    -- r_7 = false
8    OpLoadK(ABX(8, 5))    -- r_8 = "Hello World"
9    OpLoadK(ABX(9, 6))    -- r_9 = 1
10    OpLoadK(ABX(10, 7))    -- r_10 = 2
11    OpLoadK(ABX(11, 8))    -- r_11 = 3
12    OpLoadNil(ABC(12, 22, 0))    -- r_12..r_22 = nil
13    OpGetGlobal(ABX(23, 10))    -- r_23 = gbl
14    OpSetGlobal(ABX(23, 9))    -- NEW_GLOBAL = r_23
15    OpAdd(ABC(23, 4, 4))    -- r_23 = r_4 + r_4
16    OpAdd(ABC(24, 4, 262))    -- r_24 = r_4 + 1
17    OpAdd(ABC(25, 262, 4))    -- r_25 = 1 + r_4
18    OpSub(ABC(26, 4, 4))    -- r_26 = r_4 - r_4
19    OpSub(ABC(27, 4, 262))    -- r_27 = r_4 - 1
20    OpSub(ABC(28, 262, 4))    -- r_28 = 1 - r_4
21    OpMul(ABC(29, 4, 4))    -- r_29 = r_4 * r_4
22    OpMul(ABC(30, 4, 262))    -- r_30 = r_4 * 1
23    OpMul(ABC(31, 262, 4))    -- r_31 = 1 * r_4
24    OpDiv(ABC(32, 4, 4))    -- r_32 = r_4 / r_4
25    OpDiv(ABC(33, 4, 262))    -- r_33 = r_4 / 1
26    OpDiv(ABC(34, 262, 4))    -- r_34 = 1 / r_4
27    OpPow(ABC(35, 4, 4))    -- r_35 = r_4 ^ r_4
28    OpPow(ABC(36, 4, 262))    -- r_36 = r_4 ^ 1
29    OpPow(ABC(37, 262, 4))    -- r_37 = 1 ^ r_4
30    OpMod(ABC(38, 4, 4))    -- r_38 = r_4 % r_4
31    OpMod(ABC(39, 4, 262))    -- r_39 = r_4 % 1
32    OpMod(ABC(40, 262, 4))    -- r_40 = 1 % r_4
33    OpMove(ABC(41, 8, 0))    -- r_41 = r_8
34    OpMove(ABC(42, 8, 0))    -- r_42 = r_8
35    OpConcat(ABC(41, 41, 42))    -- r_41 = r_41 .. ... .. r_42
36    OpMove(ABC(42, 8, 0))    -- r_42 = r_8
37    OpMove(ABC(43, 8, 0))    -- r_43 = r_8
38    OpMove(ABC(44, 8, 0))    -- r_44 = r_8
39    OpConcat(ABC(42, 42, 44))    -- r_42 = r_42 .. ... .. r_44
40    OpMove(ABC(43, 8, 0))    -- r_43 = r_8
41    OpMove(ABC(44, 8, 0))    -- r_44 = r_8
42    OpMove(ABC(45, 8, 0))    -- r_45 = r_8
43    OpMove(ABC(46, 8, 0))    -- r_46 = r_8
44    OpConcat(ABC(43, 43, 46))    -- r_43 = r_43 .. ... .. r_46
45    OpUnm(ABC(44, 9, 0))    -- r_44 = -r_9
46    OpNot(ABC(45, 7, 0))    -- r_45 = not r_7
47    OpLen(ABC(46, 8, 0))    -- r_46 = #r_8
48    OpMove(ABC(47, 9, 0))    -- r_47 = r_9
49    OpMove(ABC(48, 10, 0))    -- r_48 = r_10
50    OpLoadK(ABX(49, 6))    -- r_49 = 1
51    OpUnknown(32)    -- TODO: DESCRIBE OpUnknown(32)
52    OpForLoop(ASBX(47, -1))    -- r_47 += r_49; if r_47 <?= r_48 then { goto 53; r_50 = r_47 }
53    OpMove(ABC(47, 9, 0))    -- r_47 = r_9
54    OpMove(ABC(48, 10, 0))    -- r_48 = r_10
55    OpMove(ABC(49, 11, 0))    -- r_49 = r_11
56    OpUnknown(32)    -- TODO: DESCRIBE OpUnknown(32)
57    OpForLoop(ASBX(47, -1))    -- r_47 += r_49; if r_47 <?= r_48 then { goto 58; r_50 = r_47 }
58    OpMove(ABC(47, 9, 0))    -- r_47 = r_9
59    OpMove(ABC(48, 10, 0))    -- r_48 = r_10
60    OpMove(ABC(49, 11, 0))    -- r_49 = r_11
61    OpUnknown(32)    -- TODO: DESCRIBE OpUnknown(32)
62    OpLoadNil(ABC(51, 51, 0))    -- r_51..r_51 = nil
63    OpForLoop(ASBX(47, -2))    -- r_47 += r_49; if r_47 <?= r_48 then { goto 63; r_50 = r_47 }
64    OpTest(ABC(7, 0, 0))    -- if r_7 then goto 66
65    OpJmp(ASBX(0, 1))    -- goto 67
66    OpJmp(ASBX(0, -3))    -- goto 64
67    OpTest(ABC(7, 0, 0))    -- if r_7 then goto 69
68    OpJmp(ASBX(0, 2))    -- goto 71
69    OpLoadNil(ABC(47, 47, 0))    -- r_47..r_47 = nil
70    OpJmp(ASBX(0, -4))    -- goto 67
71    OpTest(ABC(6, 0, 1))    -- if not r_6 then goto 73
72    OpJmp(ASBX(0, 1))    -- goto 74
73    OpJmp(ASBX(0, -3))    -- goto 71
74    OpTest(ABC(6, 0, 1))    -- if not r_6 then goto 76
75    OpJmp(ASBX(0, 2))    -- goto 78
76    OpLoadNil(ABC(47, 47, 0))    -- r_47..r_47 = nil
77    OpJmp(ASBX(0, -4))    -- goto 74
78    OpTest(ABC(6, 0, 0))    -- if r_6 then goto 80
79    OpJmp(ASBX(0, -2))    -- goto 78
80    OpLoadNil(ABC(47, 47, 0))    -- r_47..r_47 = nil
81    OpTest(ABC(6, 0, 0))    -- if r_6 then goto 83
82    OpJmp(ASBX(0, -3))    -- goto 80
83    OpTest(ABC(7, 0, 1))    -- if not r_7 then goto 85
84    OpJmp(ASBX(0, -2))    -- goto 83
85    OpLoadNil(ABC(47, 47, 0))    -- r_47..r_47 = nil
86    OpTest(ABC(7, 0, 1))    -- if not r_7 then goto 88
87    OpJmp(ASBX(0, -3))    -- goto 85
88    OpTestSet(ABC(47, 13, 1))    -- if not r_13 then goto 90 else r_47 = r_13
89    OpJmp(ASBX(0, 1))    -- goto 91
90    OpMove(ABC(47, 14, 0))    -- r_47 = r_14
91    OpTest(ABC(13, 0, 0))    -- if r_13 then goto 93
92    OpJmp(ASBX(0, 3))    -- goto 96
93    OpMove(ABC(48, 14, 0))    -- r_48 = r_14
94    OpJmp(ASBX(0, 2))    -- goto 97
95    OpLoadBool(ABC(48, 0, 1))    -- r_48 = false; pc++
96    OpLoadBool(ABC(48, 1, 0))    -- r_48 = true
97    OpEq(ABC(1, 4, 262))    -- if r_4 ~= 1 then goto 99
98    OpJmp(ASBX(0, 1))    -- goto 100
99    OpLoadBool(ABC(49, 0, 1))    -- r_49 = false; pc++
100    OpLoadBool(ABC(49, 1, 0))    -- r_49 = true
101    OpEq(ABC(1, 262, 4))    -- if 1 ~= r_4 then goto 103
102    OpJmp(ASBX(0, 1))    -- goto 104
103    OpLoadBool(ABC(50, 0, 1))    -- r_50 = false; pc++
104    OpLoadBool(ABC(50, 1, 0))    -- r_50 = true
105    OpEq(ABC(1, 4, 4))    -- if r_4 ~= r_4 then goto 107
106    OpJmp(ASBX(0, 1))    -- goto 108
107    OpLoadBool(ABC(51, 0, 1))    -- r_51 = false; pc++
108    OpLoadBool(ABC(51, 1, 0))    -- r_51 = true
109    OpEq(ABC(1, 262, 262))    -- if 1 ~= 1 then goto 111
110    OpJmp(ASBX(0, 1))    -- goto 112
111    OpLoadBool(ABC(52, 0, 1))    -- r_52 = false; pc++
112    OpLoadBool(ABC(52, 1, 0))    -- r_52 = true
113    OpEq(ABC(0, 4, 262))    -- if r_4 == 1 then goto 115
114    OpJmp(ASBX(0, 1))    -- goto 116
115    OpLoadBool(ABC(53, 0, 1))    -- r_53 = false; pc++
116    OpLoadBool(ABC(53, 1, 0))    -- r_53 = true
117    OpEq(ABC(0, 262, 4))    -- if 1 == r_4 then goto 119
118    OpJmp(ASBX(0, 1))    -- goto 120
119    OpLoadBool(ABC(54, 0, 1))    -- r_54 = false; pc++
120    OpLoadBool(ABC(54, 1, 0))    -- r_54 = true
121    OpEq(ABC(0, 4, 4))    -- if r_4 == r_4 then goto 123
122    OpJmp(ASBX(0, 1))    -- goto 124
123    OpLoadBool(ABC(55, 0, 1))    -- r_55 = false; pc++
124    OpLoadBool(ABC(55, 1, 0))    -- r_55 = true
125    OpEq(ABC(0, 262, 262))    -- if 1 == 1 then goto 127
126    OpJmp(ASBX(0, 1))    -- goto 128
127    OpLoadBool(ABC(56, 0, 1))    -- r_56 = false; pc++
128    OpLoadBool(ABC(56, 1, 0))    -- r_56 = true
129    OpLe(ABC(1, 4, 262))    -- if r_4 > 1 then goto 131
130    OpJmp(ASBX(0, 1))    -- goto 132
131    OpLoadBool(ABC(57, 0, 1))    -- r_57 = false; pc++
132    OpLoadBool(ABC(57, 1, 0))    -- r_57 = true
133    OpLe(ABC(1, 262, 4))    -- if 1 > r_4 then goto 135
134    OpJmp(ASBX(0, 1))    -- goto 136
135    OpLoadBool(ABC(58, 0, 1))    -- r_58 = false; pc++
136    OpLoadBool(ABC(58, 1, 0))    -- r_58 = true
137    OpLe(ABC(1, 4, 4))    -- if r_4 > r_4 then goto 139
138    OpJmp(ASBX(0, 1))    -- goto 140
139    OpLoadBool(ABC(59, 0, 1))    -- r_59 = false; pc++
140    OpLoadBool(ABC(59, 1, 0))    -- r_59 = true
141    OpLe(ABC(1, 262, 262))    -- if 1 > 1 then goto 143
142    OpJmp(ASBX(0, 1))    -- goto 144
143    OpLoadBool(ABC(60, 0, 1))    -- r_60 = false; pc++
144    OpLoadBool(ABC(60, 1, 0))    -- r_60 = true
145    OpLe(ABC(1, 262, 4))    -- if 1 > r_4 then goto 147
146    OpJmp(ASBX(0, 1))    -- goto 148
147    OpLoadBool(ABC(61, 0, 1))    -- r_61 = false; pc++
148    OpLoadBool(ABC(61, 1, 0))    -- r_61 = true
149    OpLe(ABC(1, 4, 262))    -- if r_4 > 1 then goto 151
150    OpJmp(ASBX(0, 1))    -- goto 152
151    OpLoadBool(ABC(62, 0, 1))    -- r_62 = false; pc++
152    OpLoadBool(ABC(62, 1, 0))    -- r_62 = true
153    OpLe(ABC(1, 4, 4))    -- if r_4 > r_4 then goto 155
154    OpJmp(ASBX(0, 1))    -- goto 156
155    OpLoadBool(ABC(63, 0, 1))    -- r_63 = false; pc++
156    OpLoadBool(ABC(63, 1, 0))    -- r_63 = true
157    OpLe(ABC(1, 262, 262))    -- if 1 > 1 then goto 159
158    OpJmp(ASBX(0, 1))    -- goto 160
159    OpLoadBool(ABC(64, 0, 1))    -- r_64 = false; pc++
160    OpLoadBool(ABC(64, 1, 0))    -- r_64 = true
161    OpMove(ABC(65, 1, 0))    -- r_65 = r_1
162    OpCall(ABC(65, 1, 1))    -- r_65()
163    OpMove(ABC(65, 1, 0))    -- r_65 = r_1
164    OpCall(ABC(65, 1, 2))    -- r_65 .. ... .. r_65 = r_65()
165    OpMove(ABC(66, 1, 0))    -- r_66 = r_1
166    OpCall(ABC(66, 1, 3))    -- r_66 .. ... .. r_67 = r_66()
167    OpMove(ABC(68, 1, 0))    -- r_68 = r_1
168    OpCall(ABC(68, 1, 4))    -- r_68 .. ... .. r_70 = r_68()
169    OpMove(ABC(71, 3, 0))    -- r_71 = r_3
170    OpMove(ABC(72, 4, 0))    -- r_72 = r_4
171    OpCall(ABC(71, 2, 1))    -- r_71(r_72 .. ... .. r_72)
172    OpMove(ABC(71, 3, 0))    -- r_71 = r_3
173    OpMove(ABC(72, 4, 0))    -- r_72 = r_4
174    OpCall(ABC(71, 2, 2))    -- r_71 .. ... .. r_71 = r_71(r_72 .. ... .. r_72)
175    OpMove(ABC(72, 3, 0))    -- r_72 = r_3
176    OpMove(ABC(73, 4, 0))    -- r_73 = r_4
177    OpCall(ABC(72, 2, 3))    -- r_72 .. ... .. r_73 = r_72(r_73 .. ... .. r_73)
178    OpMove(ABC(74, 3, 0))    -- r_74 = r_3
179    OpMove(ABC(75, 4, 0))    -- r_75 = r_4
180    OpCall(ABC(74, 2, 4))    -- r_74 .. ... .. r_76 = r_74(r_75 .. ... .. r_75)
181    OpMove(ABC(77, 1, 0))    -- r_77 = r_1
182    OpMove(ABC(78, 5, 0))    -- r_78 = r_5
183    OpMove(ABC(79, 5, 0))    -- r_79 = r_5
184    OpCall(ABC(77, 3, 1))    -- r_77(r_78 .. ... .. r_79)
185    OpMove(ABC(77, 1, 0))    -- r_77 = r_1
186    OpMove(ABC(78, 5, 0))    -- r_78 = r_5
187    OpMove(ABC(79, 5, 0))    -- r_79 = r_5
188    OpCall(ABC(77, 3, 2))    -- r_77 .. ... .. r_77 = r_77(r_78 .. ... .. r_79)
189    OpMove(ABC(78, 1, 0))    -- r_78 = r_1
190    OpMove(ABC(79, 5, 0))    -- r_79 = r_5
191    OpMove(ABC(80, 5, 0))    -- r_80 = r_5
192    OpCall(ABC(78, 3, 3))    -- r_78 .. ... .. r_79 = r_78(r_79 .. ... .. r_80)
193    OpMove(ABC(80, 1, 0))    -- r_80 = r_1
194    OpMove(ABC(81, 5, 0))    -- r_81 = r_5
195    OpMove(ABC(82, 5, 0))    -- r_82 = r_5
196    OpCall(ABC(80, 3, 4))    -- r_80 .. ... .. r_82 = r_80(r_81 .. ... .. r_82)
local function proto_0()
    0    OpGetGlobal(ABX(0, 0))    -- r_0 = n
    1    OpReturn(ABC(0, 2, 0))    -- returnr_0 .. ... .. r_0
    2    OpReturn(ABC(0, 1, 0))    -- return
end
197    OpClosure(ABX(83, 0))    -- TODO: DESCRIBE OpClosure(ABX(83, 0))
local function proto_1()
    0    OpGetUpval(ABC(0, 0, 0))    -- r_0 = upvalue_0
    1    OpReturn(ABC(0, 2, 0))    -- returnr_0 .. ... .. r_0
    2    OpReturn(ABC(0, 1, 0))    -- return
end
198    OpClosure(ABX(84, 1))    -- TODO: DESCRIBE OpClosure(ABX(84, 1))
199    OpMove(ABC(0, 8, 0))    -- r_0 = r_8
local function proto_2()
    0    OpLoadK(ABX(0, 0))    -- r_0 = "Goodbye World"
    1    OpSetUpval(ABC(0, 0, 0))    -- upvalue_0 = r_0
    2    OpGetUpval(ABC(0, 0, 0))    -- r_0 = upvalue_0
    3    OpReturn(ABC(0, 2, 0))    -- returnr_0 .. ... .. r_0
    4    OpReturn(ABC(0, 1, 0))    -- return
end
200    OpClosure(ABX(85, 2))    -- TODO: DESCRIBE OpClosure(ABX(85, 2))
201    OpMove(ABC(0, 8, 0))    -- r_0 = r_8
local function proto_3()
    0    OpGetUpval(ABC(0, 0, 0))    -- r_0 = upvalue_0
    1    OpGetUpval(ABC(1, 1, 0))    -- r_1 = upvalue_1
    2    OpReturn(ABC(0, 3, 0))    -- returnr_0 .. ... .. r_1
    3    OpReturn(ABC(0, 1, 0))    -- return
end
202    OpClosure(ABX(86, 3))    -- TODO: DESCRIBE OpClosure(ABX(86, 3))
203    OpMove(ABC(0, 9, 0))    -- r_0 = r_9
204    OpMove(ABC(0, 10, 0))    -- r_0 = r_10
local function proto_4()
    0    OpGetUpval(ABC(0, 0, 0))    -- r_0 = upvalue_0
    1    OpGetUpval(ABC(1, 1, 0))    -- r_1 = upvalue_1
    2    OpGetUpval(ABC(2, 2, 0))    -- r_2 = upvalue_2
    3    OpReturn(ABC(0, 4, 0))    -- returnr_0 .. ... .. r_2
    4    OpReturn(ABC(0, 1, 0))    -- return
end
205    OpClosure(ABX(87, 4))    -- TODO: DESCRIBE OpClosure(ABX(87, 4))
206    OpMove(ABC(0, 9, 0))    -- r_0 = r_9
207    OpMove(ABC(0, 10, 0))    -- r_0 = r_10
208    OpMove(ABC(0, 11, 0))    -- r_0 = r_11
local function proto_5()
    0    OpGetUpval(ABC(0, 0, 0))    -- r_0 = upvalue_0
    1    OpGetUpval(ABC(1, 0, 0))    -- r_1 = upvalue_0
    2    OpGetUpval(ABC(2, 0, 0))    -- r_2 = upvalue_0
    3    OpGetUpval(ABC(3, 0, 0))    -- r_3 = upvalue_0
    4    OpGetUpval(ABC(4, 0, 0))    -- r_4 = upvalue_0
    5    OpGetUpval(ABC(5, 0, 0))    -- r_5 = upvalue_0
    6    OpGetUpval(ABC(6, 0, 0))    -- r_6 = upvalue_0
    7    OpGetUpval(ABC(7, 0, 0))    -- r_7 = upvalue_0
    8    OpGetUpval(ABC(8, 0, 0))    -- r_8 = upvalue_0
    9    OpGetUpval(ABC(9, 0, 0))    -- r_9 = upvalue_0
    10    OpGetUpval(ABC(10, 0, 0))    -- r_10 = upvalue_0
    11    OpGetUpval(ABC(11, 0, 0))    -- r_11 = upvalue_0
    12    OpGetUpval(ABC(12, 0, 0))    -- r_12 = upvalue_0
    13    OpGetUpval(ABC(13, 0, 0))    -- r_13 = upvalue_0
    14    OpGetUpval(ABC(14, 0, 0))    -- r_14 = upvalue_0
    15    OpGetUpval(ABC(15, 0, 0))    -- r_15 = upvalue_0
    16    OpGetUpval(ABC(16, 0, 0))    -- r_16 = upvalue_0
    17    OpGetUpval(ABC(17, 0, 0))    -- r_17 = upvalue_0
    18    OpGetUpval(ABC(18, 0, 0))    -- r_18 = upvalue_0
    19    OpReturn(ABC(0, 20, 0))    -- returnr_0 .. ... .. r_18
    20    OpReturn(ABC(0, 1, 0))    -- return
end
209    OpClosure(ABX(88, 5))    -- TODO: DESCRIBE OpClosure(ABX(88, 5))
210    OpMove(ABC(0, 9, 0))    -- r_0 = r_9
local function proto_6()
    0    OpGetUpval(ABC(1, 0, 0))    -- r_1 = upvalue_0
    1    OpUnknown(37)    -- TODO: DESCRIBE OpUnknown(37)
    2    OpTailCall(ABC(1, 0, 0))    -- return r_1(top ... ???)
    3    OpReturn(ABC(1, 0, 0))    -- return top ... ???
    4    OpReturn(ABC(0, 1, 0))    -- return
end
211    OpClosure(ABX(89, 6))    -- TODO: DESCRIBE OpClosure(ABX(89, 6))
212    OpMove(ABC(0, 86, 0))    -- r_0 = r_86
local function proto_7()
    0    OpGetUpval(ABC(0, 0, 0))    -- r_0 = upvalue_0
    1    OpGetUpval(ABC(1, 1, 0))    -- r_1 = upvalue_1
    2    OpTailCall(ABC(0, 2, 0))    -- return r_0(r_1 .. ... .. r_1)
    3    OpReturn(ABC(0, 0, 0))    -- return top ... ???
    4    OpReturn(ABC(0, 1, 0))    -- return
end
213    OpClosure(ABX(90, 7))    -- TODO: DESCRIBE OpClosure(ABX(90, 7))
214    OpMove(ABC(0, 86, 0))    -- r_0 = r_86
215    OpMove(ABC(0, 5, 0))    -- r_0 = r_5
local function proto_8()
    0    OpGetUpval(ABC(0, 0, 0))    -- r_0 = upvalue_0
    1    OpGetUpval(ABC(1, 1, 0))    -- r_1 = upvalue_1
    2    OpGetUpval(ABC(2, 1, 0))    -- r_2 = upvalue_1
    3    OpTailCall(ABC(0, 3, 0))    -- return r_0(r_1 .. ... .. r_2)
    4    OpReturn(ABC(0, 0, 0))    -- return top ... ???
    5    OpReturn(ABC(0, 1, 0))    -- return
end
216    OpClosure(ABX(91, 8))    -- TODO: DESCRIBE OpClosure(ABX(91, 8))
217    OpMove(ABC(0, 86, 0))    -- r_0 = r_86
218    OpMove(ABC(0, 5, 0))    -- r_0 = r_5
219    OpMove(ABC(92, 83, 0))    -- r_92 = r_83
220    OpCall(ABC(92, 1, 1))    -- r_92()
221    OpMove(ABC(92, 84, 0))    -- r_92 = r_84
222    OpCall(ABC(92, 1, 2))    -- r_92 .. ... .. r_92 = r_92()
223    OpMove(ABC(93, 85, 0))    -- r_93 = r_85
224    OpCall(ABC(93, 1, 1))    -- r_93()
225    OpMove(ABC(93, 86, 0))    -- r_93 = r_86
226    OpCall(ABC(93, 1, 3))    -- r_93 .. ... .. r_94 = r_93()
227    OpMove(ABC(95, 87, 0))    -- r_95 = r_87
228    OpCall(ABC(95, 1, 4))    -- r_95 .. ... .. r_97 = r_95()
229    OpMove(ABC(98, 89, 0))    -- r_98 = r_89
230    OpCall(ABC(98, 1, 2))    -- r_98 .. ... .. r_98 = r_98()
231    OpMove(ABC(99, 90, 0))    -- r_99 = r_90
232    OpCall(ABC(99, 1, 3))    -- r_99 .. ... .. r_100 = r_99()
233    OpMove(ABC(101, 91, 0))    -- r_101 = r_91
234    OpCall(ABC(101, 1, 3))    -- r_101 .. ... .. r_102 = r_101()
235    OpMove(ABC(103, 88, 0))    -- r_103 = r_88
236    OpCall(ABC(103, 1, 10))    -- r_103 .. ... .. r_111 = r_103()
237    OpMove(ABC(112, 88, 0))    -- r_112 = r_88
238    OpMove(ABC(113, 5, 0))    -- r_113 = r_5
239    OpMove(ABC(114, 5, 0))    -- r_114 = r_5
240    OpMove(ABC(115, 5, 0))    -- r_115 = r_5
241    OpMove(ABC(116, 5, 0))    -- r_116 = r_5
242    OpMove(ABC(117, 5, 0))    -- r_117 = r_5
243    OpMove(ABC(118, 5, 0))    -- r_118 = r_5
244    OpMove(ABC(119, 5, 0))    -- r_119 = r_5
245    OpMove(ABC(120, 5, 0))    -- r_120 = r_5
246    OpMove(ABC(121, 5, 0))    -- r_121 = r_5
247    OpMove(ABC(122, 5, 0))    -- r_122 = r_5
248    OpMove(ABC(123, 5, 0))    -- r_123 = r_5
249    OpMove(ABC(124, 5, 0))    -- r_124 = r_5
250    OpMove(ABC(125, 5, 0))    -- r_125 = r_5
251    OpMove(ABC(126, 5, 0))    -- r_126 = r_5
252    OpMove(ABC(127, 5, 0))    -- r_127 = r_5
253    OpMove(ABC(128, 5, 0))    -- r_128 = r_5
254    OpCall(ABC(112, 17, 1))    -- r_112(r_113 .. ... .. r_128)
255    OpMove(ABC(112, 88, 0))    -- r_112 = r_88
256    OpCall(ABC(112, 1, 10))    -- r_112 .. ... .. r_120 = r_112()
257    OpMove(ABC(121, 88, 0))    -- r_121 = r_88
258    OpMove(ABC(122, 5, 0))    -- r_122 = r_5
259    OpMove(ABC(123, 5, 0))    -- r_123 = r_5
260    OpMove(ABC(124, 5, 0))    -- r_124 = r_5
261    OpMove(ABC(125, 5, 0))    -- r_125 = r_5
262    OpMove(ABC(126, 5, 0))    -- r_126 = r_5
263    OpMove(ABC(127, 5, 0))    -- r_127 = r_5
264    OpMove(ABC(128, 5, 0))    -- r_128 = r_5
265    OpMove(ABC(129, 5, 0))    -- r_129 = r_5
266    OpMove(ABC(130, 5, 0))    -- r_130 = r_5
267    OpMove(ABC(131, 5, 0))    -- r_131 = r_5
268    OpMove(ABC(132, 5, 0))    -- r_132 = r_5
269    OpMove(ABC(133, 5, 0))    -- r_133 = r_5
270    OpMove(ABC(134, 5, 0))    -- r_134 = r_5
271    OpMove(ABC(135, 5, 0))    -- r_135 = r_5
272    OpMove(ABC(136, 5, 0))    -- r_136 = r_5
273    OpMove(ABC(137, 5, 0))    -- r_137 = r_5
274    OpCall(ABC(121, 17, 1))    -- r_121(r_122 .. ... .. r_137)
275    OpNewTable(ABC(121, 0, 0))    -- r_121 = {} -- 0 list & 0 record
276    OpNewTable(ABC(122, 3, 0))    -- r_122 = {} -- 3 list & 0 record
277    OpMove(ABC(123, 9, 0))    -- r_123 = r_9
278    OpMove(ABC(124, 10, 0))    -- r_124 = r_10
279    OpMove(ABC(125, 11, 0))    -- r_125 = r_11
280    OpUnknown(34)    -- TODO: DESCRIBE OpUnknown(34)
281    OpMove(ABC(121, 122, 0))    -- r_121 = r_122
282    OpNewTable(ABC(122, 1, 0))    -- r_122 = {} -- 1 list & 0 record
283    OpMove(ABC(123, 9, 0))    -- r_123 = r_9
284    OpUnknown(34)    -- TODO: DESCRIBE OpUnknown(34)
285    OpMove(ABC(121, 122, 0))    -- r_121 = r_122
286    OpNewTable(ABC(122, 29, 0))    -- r_122 = {} -- 29 list & 0 record
287    OpMove(ABC(123, 9, 0))    -- r_123 = r_9
288    OpMove(ABC(124, 10, 0))    -- r_124 = r_10
289    OpMove(ABC(125, 11, 0))    -- r_125 = r_11
290    OpMove(ABC(126, 9, 0))    -- r_126 = r_9
291    OpMove(ABC(127, 10, 0))    -- r_127 = r_10
292    OpMove(ABC(128, 11, 0))    -- r_128 = r_11
293    OpMove(ABC(129, 9, 0))    -- r_129 = r_9
294    OpMove(ABC(130, 10, 0))    -- r_130 = r_10
295    OpMove(ABC(131, 11, 0))    -- r_131 = r_11
296    OpMove(ABC(132, 9, 0))    -- r_132 = r_9
297    OpMove(ABC(133, 10, 0))    -- r_133 = r_10
298    OpMove(ABC(134, 11, 0))    -- r_134 = r_11
299    OpMove(ABC(135, 9, 0))    -- r_135 = r_9
300    OpMove(ABC(136, 10, 0))    -- r_136 = r_10
301    OpMove(ABC(137, 11, 0))    -- r_137 = r_11
302    OpMove(ABC(138, 9, 0))    -- r_138 = r_9
303    OpMove(ABC(139, 10, 0))    -- r_139 = r_10
304    OpMove(ABC(140, 11, 0))    -- r_140 = r_11
305    OpMove(ABC(141, 9, 0))    -- r_141 = r_9
306    OpMove(ABC(142, 10, 0))    -- r_142 = r_10
307    OpMove(ABC(143, 11, 0))    -- r_143 = r_11
308    OpMove(ABC(144, 9, 0))    -- r_144 = r_9
309    OpMove(ABC(145, 10, 0))    -- r_145 = r_10
310    OpMove(ABC(146, 11, 0))    -- r_146 = r_11
311    OpMove(ABC(147, 9, 0))    -- r_147 = r_9
312    OpMove(ABC(148, 9, 0))    -- r_148 = r_9
313    OpMove(ABC(149, 10, 0))    -- r_149 = r_10
314    OpMove(ABC(150, 11, 0))    -- r_150 = r_11
315    OpMove(ABC(151, 9, 0))    -- r_151 = r_9
316    OpMove(ABC(152, 10, 0))    -- r_152 = r_10
317    OpMove(ABC(153, 11, 0))    -- r_153 = r_11
318    OpMove(ABC(154, 9, 0))    -- r_154 = r_9
319    OpMove(ABC(155, 10, 0))    -- r_155 = r_10
320    OpMove(ABC(156, 11, 0))    -- r_156 = r_11
321    OpMove(ABC(157, 9, 0))    -- r_157 = r_9
322    OpMove(ABC(158, 10, 0))    -- r_158 = r_10
323    OpMove(ABC(159, 11, 0))    -- r_159 = r_11
324    OpMove(ABC(160, 9, 0))    -- r_160 = r_9
325    OpMove(ABC(161, 10, 0))    -- r_161 = r_10
326    OpMove(ABC(162, 11, 0))    -- r_162 = r_11
327    OpMove(ABC(163, 9, 0))    -- r_163 = r_9
328    OpMove(ABC(164, 10, 0))    -- r_164 = r_10
329    OpMove(ABC(165, 11, 0))    -- r_165 = r_11
330    OpMove(ABC(166, 9, 0))    -- r_166 = r_9
331    OpMove(ABC(167, 10, 0))    -- r_167 = r_10
332    OpMove(ABC(168, 11, 0))    -- r_168 = r_11
333    OpMove(ABC(169, 9, 0))    -- r_169 = r_9
334    OpMove(ABC(170, 10, 0))    -- r_170 = r_10
335    OpMove(ABC(171, 11, 0))    -- r_171 = r_11
336    OpMove(ABC(172, 9, 0))    -- r_172 = r_9
337    OpUnknown(34)    -- TODO: DESCRIBE OpUnknown(34)
338    OpMove(ABC(121, 122, 0))    -- r_121 = r_122
339    OpNewTable(ABC(122, 29, 0))    -- r_122 = {} -- 29 list & 0 record
340    OpMove(ABC(123, 9, 0))    -- r_123 = r_9
341    OpMove(ABC(124, 10, 0))    -- r_124 = r_10
342    OpMove(ABC(125, 11, 0))    -- r_125 = r_11
343    OpMove(ABC(126, 9, 0))    -- r_126 = r_9
344    OpMove(ABC(127, 10, 0))    -- r_127 = r_10
345    OpMove(ABC(128, 11, 0))    -- r_128 = r_11
346    OpMove(ABC(129, 9, 0))    -- r_129 = r_9
347    OpMove(ABC(130, 10, 0))    -- r_130 = r_10
348    OpMove(ABC(131, 11, 0))    -- r_131 = r_11
349    OpMove(ABC(132, 9, 0))    -- r_132 = r_9
350    OpMove(ABC(133, 10, 0))    -- r_133 = r_10
351    OpMove(ABC(134, 11, 0))    -- r_134 = r_11
352    OpMove(ABC(135, 9, 0))    -- r_135 = r_9
353    OpMove(ABC(136, 10, 0))    -- r_136 = r_10
354    OpMove(ABC(137, 11, 0))    -- r_137 = r_11
355    OpMove(ABC(138, 9, 0))    -- r_138 = r_9
356    OpMove(ABC(139, 10, 0))    -- r_139 = r_10
357    OpMove(ABC(140, 11, 0))    -- r_140 = r_11
358    OpMove(ABC(141, 9, 0))    -- r_141 = r_9
359    OpMove(ABC(142, 10, 0))    -- r_142 = r_10
360    OpMove(ABC(143, 11, 0))    -- r_143 = r_11
361    OpMove(ABC(144, 9, 0))    -- r_144 = r_9
362    OpMove(ABC(145, 10, 0))    -- r_145 = r_10
363    OpMove(ABC(146, 11, 0))    -- r_146 = r_11
364    OpMove(ABC(147, 9, 0))    -- r_147 = r_9
365    OpMove(ABC(148, 9, 0))    -- r_148 = r_9
366    OpMove(ABC(149, 10, 0))    -- r_149 = r_10
367    OpMove(ABC(150, 11, 0))    -- r_150 = r_11
368    OpMove(ABC(151, 9, 0))    -- r_151 = r_9
369    OpMove(ABC(152, 10, 0))    -- r_152 = r_10
370    OpMove(ABC(153, 11, 0))    -- r_153 = r_11
371    OpMove(ABC(154, 9, 0))    -- r_154 = r_9
372    OpMove(ABC(155, 10, 0))    -- r_155 = r_10
373    OpMove(ABC(156, 11, 0))    -- r_156 = r_11
374    OpMove(ABC(157, 9, 0))    -- r_157 = r_9
375    OpMove(ABC(158, 10, 0))    -- r_158 = r_10
376    OpMove(ABC(159, 11, 0))    -- r_159 = r_11
377    OpMove(ABC(160, 9, 0))    -- r_160 = r_9
378    OpMove(ABC(161, 10, 0))    -- r_161 = r_10
379    OpMove(ABC(162, 11, 0))    -- r_162 = r_11
380    OpMove(ABC(163, 9, 0))    -- r_163 = r_9
381    OpMove(ABC(164, 10, 0))    -- r_164 = r_10
382    OpMove(ABC(165, 11, 0))    -- r_165 = r_11
383    OpMove(ABC(166, 9, 0))    -- r_166 = r_9
384    OpMove(ABC(167, 10, 0))    -- r_167 = r_10
385    OpMove(ABC(168, 11, 0))    -- r_168 = r_11
386    OpMove(ABC(169, 9, 0))    -- r_169 = r_9
387    OpMove(ABC(170, 10, 0))    -- r_170 = r_10
388    OpMove(ABC(171, 11, 0))    -- r_171 = r_11
389    OpMove(ABC(172, 9, 0))    -- r_172 = r_9
390    OpUnknown(34)    -- TODO: DESCRIBE OpUnknown(34)
391    OpMove(ABC(123, 9, 0))    -- r_123 = r_9
392    OpUnknown(34)    -- TODO: DESCRIBE OpUnknown(34)
393    OpMove(ABC(121, 122, 0))    -- r_121 = r_122
394    OpNewTable(ABC(122, 37, 0))    -- r_122 = {} -- 37 list & 0 record
395    OpMove(ABC(123, 9, 0))    -- r_123 = r_9
396    OpMove(ABC(124, 10, 0))    -- r_124 = r_10
397    OpMove(ABC(125, 11, 0))    -- r_125 = r_11
398    OpMove(ABC(126, 9, 0))    -- r_126 = r_9
399    OpMove(ABC(127, 10, 0))    -- r_127 = r_10
400    OpMove(ABC(128, 11, 0))    -- r_128 = r_11
401    OpMove(ABC(129, 9, 0))    -- r_129 = r_9
402    OpMove(ABC(130, 10, 0))    -- r_130 = r_10
403    OpMove(ABC(131, 11, 0))    -- r_131 = r_11
404    OpMove(ABC(132, 9, 0))    -- r_132 = r_9
405    OpMove(ABC(133, 10, 0))    -- r_133 = r_10
406    OpMove(ABC(134, 11, 0))    -- r_134 = r_11
407    OpMove(ABC(135, 9, 0))    -- r_135 = r_9
408    OpMove(ABC(136, 10, 0))    -- r_136 = r_10
409    OpMove(ABC(137, 11, 0))    -- r_137 = r_11
410    OpMove(ABC(138, 9, 0))    -- r_138 = r_9
411    OpMove(ABC(139, 10, 0))    -- r_139 = r_10
412    OpMove(ABC(140, 11, 0))    -- r_140 = r_11
413    OpMove(ABC(141, 9, 0))    -- r_141 = r_9
414    OpMove(ABC(142, 10, 0))    -- r_142 = r_10
415    OpMove(ABC(143, 11, 0))    -- r_143 = r_11
416    OpMove(ABC(144, 9, 0))    -- r_144 = r_9
417    OpMove(ABC(145, 10, 0))    -- r_145 = r_10
418    OpMove(ABC(146, 11, 0))    -- r_146 = r_11
419    OpMove(ABC(147, 9, 0))    -- r_147 = r_9
420    OpMove(ABC(148, 9, 0))    -- r_148 = r_9
421    OpMove(ABC(149, 10, 0))    -- r_149 = r_10
422    OpMove(ABC(150, 11, 0))    -- r_150 = r_11
423    OpMove(ABC(151, 9, 0))    -- r_151 = r_9
424    OpMove(ABC(152, 10, 0))    -- r_152 = r_10
425    OpMove(ABC(153, 11, 0))    -- r_153 = r_11
426    OpMove(ABC(154, 9, 0))    -- r_154 = r_9
427    OpMove(ABC(155, 10, 0))    -- r_155 = r_10
428    OpMove(ABC(156, 11, 0))    -- r_156 = r_11
429    OpMove(ABC(157, 9, 0))    -- r_157 = r_9
430    OpMove(ABC(158, 10, 0))    -- r_158 = r_10
431    OpMove(ABC(159, 11, 0))    -- r_159 = r_11
432    OpMove(ABC(160, 9, 0))    -- r_160 = r_9
433    OpMove(ABC(161, 10, 0))    -- r_161 = r_10
434    OpMove(ABC(162, 11, 0))    -- r_162 = r_11
435    OpMove(ABC(163, 9, 0))    -- r_163 = r_9
436    OpMove(ABC(164, 10, 0))    -- r_164 = r_10
437    OpMove(ABC(165, 11, 0))    -- r_165 = r_11
438    OpMove(ABC(166, 9, 0))    -- r_166 = r_9
439    OpMove(ABC(167, 10, 0))    -- r_167 = r_10
440    OpMove(ABC(168, 11, 0))    -- r_168 = r_11
441    OpMove(ABC(169, 9, 0))    -- r_169 = r_9
442    OpMove(ABC(170, 10, 0))    -- r_170 = r_10
443    OpMove(ABC(171, 11, 0))    -- r_171 = r_11
444    OpMove(ABC(172, 9, 0))    -- r_172 = r_9
445    OpUnknown(34)    -- TODO: DESCRIBE OpUnknown(34)
446    OpMove(ABC(123, 9, 0))    -- r_123 = r_9
447    OpMove(ABC(124, 10, 0))    -- r_124 = r_10
448    OpMove(ABC(125, 11, 0))    -- r_125 = r_11
449    OpMove(ABC(126, 9, 0))    -- r_126 = r_9
450    OpMove(ABC(127, 10, 0))    -- r_127 = r_10
451    OpMove(ABC(128, 11, 0))    -- r_128 = r_11
452    OpMove(ABC(129, 9, 0))    -- r_129 = r_9
453    OpMove(ABC(130, 10, 0))    -- r_130 = r_10
454    OpMove(ABC(131, 11, 0))    -- r_131 = r_11
455    OpMove(ABC(132, 9, 0))    -- r_132 = r_9
456    OpMove(ABC(133, 10, 0))    -- r_133 = r_10
457    OpMove(ABC(134, 11, 0))    -- r_134 = r_11
458    OpMove(ABC(135, 9, 0))    -- r_135 = r_9
459    OpMove(ABC(136, 10, 0))    -- r_136 = r_10
460    OpMove(ABC(137, 11, 0))    -- r_137 = r_11
461    OpMove(ABC(138, 9, 0))    -- r_138 = r_9
462    OpMove(ABC(139, 10, 0))    -- r_139 = r_10
463    OpMove(ABC(140, 11, 0))    -- r_140 = r_11
464    OpMove(ABC(141, 9, 0))    -- r_141 = r_9
465    OpMove(ABC(142, 10, 0))    -- r_142 = r_10
466    OpMove(ABC(143, 11, 0))    -- r_143 = r_11
467    OpMove(ABC(144, 9, 0))    -- r_144 = r_9
468    OpMove(ABC(145, 10, 0))    -- r_145 = r_10
469    OpMove(ABC(146, 11, 0))    -- r_146 = r_11
470    OpMove(ABC(147, 9, 0))    -- r_147 = r_9
471    OpMove(ABC(148, 9, 0))    -- r_148 = r_9
472    OpMove(ABC(149, 10, 0))    -- r_149 = r_10
473    OpMove(ABC(150, 11, 0))    -- r_150 = r_11
474    OpMove(ABC(151, 9, 0))    -- r_151 = r_9
475    OpMove(ABC(152, 10, 0))    -- r_152 = r_10
476    OpMove(ABC(153, 11, 0))    -- r_153 = r_11
477    OpMove(ABC(154, 9, 0))    -- r_154 = r_9
478    OpMove(ABC(155, 10, 0))    -- r_155 = r_10
479    OpMove(ABC(156, 11, 0))    -- r_156 = r_11
480    OpMove(ABC(157, 9, 0))    -- r_157 = r_9
481    OpMove(ABC(158, 10, 0))    -- r_158 = r_10
482    OpMove(ABC(159, 11, 0))    -- r_159 = r_11
483    OpMove(ABC(160, 9, 0))    -- r_160 = r_9
484    OpMove(ABC(161, 10, 0))    -- r_161 = r_10
485    OpMove(ABC(162, 11, 0))    -- r_162 = r_11
486    OpMove(ABC(163, 9, 0))    -- r_163 = r_9
487    OpMove(ABC(164, 10, 0))    -- r_164 = r_10
488    OpMove(ABC(165, 11, 0))    -- r_165 = r_11
489    OpMove(ABC(166, 9, 0))    -- r_166 = r_9
490    OpMove(ABC(167, 10, 0))    -- r_167 = r_10
491    OpMove(ABC(168, 11, 0))    -- r_168 = r_11
492    OpMove(ABC(169, 9, 0))    -- r_169 = r_9
493    OpMove(ABC(170, 10, 0))    -- r_170 = r_10
494    OpMove(ABC(171, 11, 0))    -- r_171 = r_11
495    OpMove(ABC(172, 9, 0))    -- r_172 = r_9
496    OpUnknown(34)    -- TODO: DESCRIBE OpUnknown(34)
497    OpMove(ABC(121, 122, 0))    -- r_121 = r_122
498    OpNewTable(ABC(122, 47, 0))    -- r_122 = {} -- 47 list & 0 record
499    OpMove(ABC(123, 9, 0))    -- r_123 = r_9
500    OpMove(ABC(124, 10, 0))    -- r_124 = r_10
501    OpMove(ABC(125, 11, 0))    -- r_125 = r_11
502    OpMove(ABC(126, 9, 0))    -- r_126 = r_9
503    OpMove(ABC(127, 10, 0))    -- r_127 = r_10
504    OpMove(ABC(128, 11, 0))    -- r_128 = r_11
505    OpMove(ABC(129, 9, 0))    -- r_129 = r_9
506    OpMove(ABC(130, 10, 0))    -- r_130 = r_10
507    OpMove(ABC(131, 11, 0))    -- r_131 = r_11
508    OpMove(ABC(132, 9, 0))    -- r_132 = r_9
509    OpMove(ABC(133, 10, 0))    -- r_133 = r_10
510    OpMove(ABC(134, 11, 0))    -- r_134 = r_11
511    OpMove(ABC(135, 9, 0))    -- r_135 = r_9
512    OpMove(ABC(136, 10, 0))    -- r_136 = r_10
513    OpMove(ABC(137, 11, 0))    -- r_137 = r_11
514    OpMove(ABC(138, 9, 0))    -- r_138 = r_9
515    OpMove(ABC(139, 10, 0))    -- r_139 = r_10
516    OpMove(ABC(140, 11, 0))    -- r_140 = r_11
517    OpMove(ABC(141, 9, 0))    -- r_141 = r_9
518    OpMove(ABC(142, 10, 0))    -- r_142 = r_10
519    OpMove(ABC(143, 11, 0))    -- r_143 = r_11
520    OpMove(ABC(144, 9, 0))    -- r_144 = r_9
521    OpMove(ABC(145, 10, 0))    -- r_145 = r_10
522    OpMove(ABC(146, 11, 0))    -- r_146 = r_11
523    OpMove(ABC(147, 9, 0))    -- r_147 = r_9
524    OpMove(ABC(148, 9, 0))    -- r_148 = r_9
525    OpMove(ABC(149, 10, 0))    -- r_149 = r_10
526    OpMove(ABC(150, 11, 0))    -- r_150 = r_11
527    OpMove(ABC(151, 9, 0))    -- r_151 = r_9
528    OpMove(ABC(152, 10, 0))    -- r_152 = r_10
529    OpMove(ABC(153, 11, 0))    -- r_153 = r_11
530    OpMove(ABC(154, 9, 0))    -- r_154 = r_9
531    OpMove(ABC(155, 10, 0))    -- r_155 = r_10
532    OpMove(ABC(156, 11, 0))    -- r_156 = r_11
533    OpMove(ABC(157, 9, 0))    -- r_157 = r_9
534    OpMove(ABC(158, 10, 0))    -- r_158 = r_10
535    OpMove(ABC(159, 11, 0))    -- r_159 = r_11
536    OpMove(ABC(160, 9, 0))    -- r_160 = r_9
537    OpMove(ABC(161, 10, 0))    -- r_161 = r_10
538    OpMove(ABC(162, 11, 0))    -- r_162 = r_11
539    OpMove(ABC(163, 9, 0))    -- r_163 = r_9
540    OpMove(ABC(164, 10, 0))    -- r_164 = r_10
541    OpMove(ABC(165, 11, 0))    -- r_165 = r_11
542    OpMove(ABC(166, 9, 0))    -- r_166 = r_9
543    OpMove(ABC(167, 10, 0))    -- r_167 = r_10
544    OpMove(ABC(168, 11, 0))    -- r_168 = r_11
545    OpMove(ABC(169, 9, 0))    -- r_169 = r_9
546    OpMove(ABC(170, 10, 0))    -- r_170 = r_10
547    OpMove(ABC(171, 11, 0))    -- r_171 = r_11
548    OpMove(ABC(172, 9, 0))    -- r_172 = r_9
549    OpUnknown(34)    -- TODO: DESCRIBE OpUnknown(34)
550    OpMove(ABC(123, 9, 0))    -- r_123 = r_9
551    OpMove(ABC(124, 10, 0))    -- r_124 = r_10
552    OpMove(ABC(125, 11, 0))    -- r_125 = r_11
553    OpMove(ABC(126, 9, 0))    -- r_126 = r_9
554    OpMove(ABC(127, 10, 0))    -- r_127 = r_10
555    OpMove(ABC(128, 11, 0))    -- r_128 = r_11
556    OpMove(ABC(129, 9, 0))    -- r_129 = r_9
557    OpMove(ABC(130, 10, 0))    -- r_130 = r_10
558    OpMove(ABC(131, 11, 0))    -- r_131 = r_11
559    OpMove(ABC(132, 9, 0))    -- r_132 = r_9
560    OpMove(ABC(133, 10, 0))    -- r_133 = r_10
561    OpMove(ABC(134, 11, 0))    -- r_134 = r_11
562    OpMove(ABC(135, 9, 0))    -- r_135 = r_9
563    OpMove(ABC(136, 10, 0))    -- r_136 = r_10
564    OpMove(ABC(137, 11, 0))    -- r_137 = r_11
565    OpMove(ABC(138, 9, 0))    -- r_138 = r_9
566    OpMove(ABC(139, 10, 0))    -- r_139 = r_10
567    OpMove(ABC(140, 11, 0))    -- r_140 = r_11
568    OpMove(ABC(141, 9, 0))    -- r_141 = r_9
569    OpMove(ABC(142, 10, 0))    -- r_142 = r_10
570    OpMove(ABC(143, 11, 0))    -- r_143 = r_11
571    OpMove(ABC(144, 9, 0))    -- r_144 = r_9
572    OpMove(ABC(145, 10, 0))    -- r_145 = r_10
573    OpMove(ABC(146, 11, 0))    -- r_146 = r_11
574    OpMove(ABC(147, 9, 0))    -- r_147 = r_9
575    OpMove(ABC(148, 9, 0))    -- r_148 = r_9
576    OpMove(ABC(149, 10, 0))    -- r_149 = r_10
577    OpMove(ABC(150, 11, 0))    -- r_150 = r_11
578    OpMove(ABC(151, 9, 0))    -- r_151 = r_9
579    OpMove(ABC(152, 10, 0))    -- r_152 = r_10
580    OpMove(ABC(153, 11, 0))    -- r_153 = r_11
581    OpMove(ABC(154, 9, 0))    -- r_154 = r_9
582    OpMove(ABC(155, 10, 0))    -- r_155 = r_10
583    OpMove(ABC(156, 11, 0))    -- r_156 = r_11
584    OpMove(ABC(157, 9, 0))    -- r_157 = r_9
585    OpMove(ABC(158, 10, 0))    -- r_158 = r_10
586    OpMove(ABC(159, 11, 0))    -- r_159 = r_11
587    OpMove(ABC(160, 9, 0))    -- r_160 = r_9
588    OpMove(ABC(161, 10, 0))    -- r_161 = r_10
589    OpMove(ABC(162, 11, 0))    -- r_162 = r_11
590    OpMove(ABC(163, 9, 0))    -- r_163 = r_9
591    OpMove(ABC(164, 10, 0))    -- r_164 = r_10
592    OpMove(ABC(165, 11, 0))    -- r_165 = r_11
593    OpMove(ABC(166, 9, 0))    -- r_166 = r_9
594    OpMove(ABC(167, 10, 0))    -- r_167 = r_10
595    OpMove(ABC(168, 11, 0))    -- r_168 = r_11
596    OpMove(ABC(169, 9, 0))    -- r_169 = r_9
597    OpMove(ABC(170, 10, 0))    -- r_170 = r_10
598    OpMove(ABC(171, 11, 0))    -- r_171 = r_11
599    OpMove(ABC(172, 9, 0))    -- r_172 = r_9
600    OpUnknown(34)    -- TODO: DESCRIBE OpUnknown(34)
601    OpMove(ABC(123, 9, 0))    -- r_123 = r_9
602    OpMove(ABC(124, 10, 0))    -- r_124 = r_10
603    OpMove(ABC(125, 11, 0))    -- r_125 = r_11
604    OpMove(ABC(126, 9, 0))    -- r_126 = r_9
605    OpMove(ABC(127, 10, 0))    -- r_127 = r_10
606    OpMove(ABC(128, 11, 0))    -- r_128 = r_11
607    OpMove(ABC(129, 9, 0))    -- r_129 = r_9
608    OpMove(ABC(130, 10, 0))    -- r_130 = r_10
609    OpMove(ABC(131, 11, 0))    -- r_131 = r_11
610    OpMove(ABC(132, 9, 0))    -- r_132 = r_9
611    OpMove(ABC(133, 10, 0))    -- r_133 = r_10
612    OpMove(ABC(134, 11, 0))    -- r_134 = r_11
613    OpMove(ABC(135, 9, 0))    -- r_135 = r_9
614    OpMove(ABC(136, 10, 0))    -- r_136 = r_10
615    OpMove(ABC(137, 11, 0))    -- r_137 = r_11
616    OpMove(ABC(138, 9, 0))    -- r_138 = r_9
617    OpMove(ABC(139, 10, 0))    -- r_139 = r_10
618    OpMove(ABC(140, 11, 0))    -- r_140 = r_11
619    OpMove(ABC(141, 9, 0))    -- r_141 = r_9
620    OpMove(ABC(142, 10, 0))    -- r_142 = r_10
621    OpMove(ABC(143, 11, 0))    -- r_143 = r_11
622    OpMove(ABC(144, 9, 0))    -- r_144 = r_9
623    OpMove(ABC(145, 10, 0))    -- r_145 = r_10
624    OpMove(ABC(146, 11, 0))    -- r_146 = r_11
625    OpMove(ABC(147, 9, 0))    -- r_147 = r_9
626    OpMove(ABC(148, 9, 0))    -- r_148 = r_9
627    OpMove(ABC(149, 10, 0))    -- r_149 = r_10
628    OpMove(ABC(150, 11, 0))    -- r_150 = r_11
629    OpMove(ABC(151, 9, 0))    -- r_151 = r_9
630    OpMove(ABC(152, 10, 0))    -- r_152 = r_10
631    OpMove(ABC(153, 11, 0))    -- r_153 = r_11
632    OpMove(ABC(154, 9, 0))    -- r_154 = r_9
633    OpMove(ABC(155, 10, 0))    -- r_155 = r_10
634    OpMove(ABC(156, 11, 0))    -- r_156 = r_11
635    OpMove(ABC(157, 9, 0))    -- r_157 = r_9
636    OpMove(ABC(158, 10, 0))    -- r_158 = r_10
637    OpMove(ABC(159, 11, 0))    -- r_159 = r_11
638    OpMove(ABC(160, 9, 0))    -- r_160 = r_9
639    OpMove(ABC(161, 10, 0))    -- r_161 = r_10
640    OpMove(ABC(162, 11, 0))    -- r_162 = r_11
641    OpMove(ABC(163, 9, 0))    -- r_163 = r_9
642    OpMove(ABC(164, 10, 0))    -- r_164 = r_10
643    OpMove(ABC(165, 11, 0))    -- r_165 = r_11
644    OpMove(ABC(166, 9, 0))    -- r_166 = r_9
645    OpMove(ABC(167, 10, 0))    -- r_167 = r_10
646    OpMove(ABC(168, 11, 0))    -- r_168 = r_11
647    OpMove(ABC(169, 9, 0))    -- r_169 = r_9
648    OpMove(ABC(170, 10, 0))    -- r_170 = r_10
649    OpMove(ABC(171, 11, 0))    -- r_171 = r_11
650    OpMove(ABC(172, 9, 0))    -- r_172 = r_9
651    OpUnknown(34)    -- TODO: DESCRIBE OpUnknown(34)
652    OpMove(ABC(123, 9, 0))    -- r_123 = r_9
653    OpMove(ABC(124, 10, 0))    -- r_124 = r_10
654    OpMove(ABC(125, 11, 0))    -- r_125 = r_11
655    OpMove(ABC(126, 9, 0))    -- r_126 = r_9
656    OpMove(ABC(127, 10, 0))    -- r_127 = r_10
657    OpMove(ABC(128, 11, 0))    -- r_128 = r_11
658    OpMove(ABC(129, 9, 0))    -- r_129 = r_9
659    OpMove(ABC(130, 10, 0))    -- r_130 = r_10
660    OpMove(ABC(131, 11, 0))    -- r_131 = r_11
661    OpMove(ABC(132, 9, 0))    -- r_132 = r_9
662    OpMove(ABC(133, 10, 0))    -- r_133 = r_10
663    OpMove(ABC(134, 11, 0))    -- r_134 = r_11
664    OpMove(ABC(135, 9, 0))    -- r_135 = r_9
665    OpMove(ABC(136, 10, 0))    -- r_136 = r_10
666    OpMove(ABC(137, 11, 0))    -- r_137 = r_11
667    OpMove(ABC(138, 9, 0))    -- r_138 = r_9
668    OpMove(ABC(139, 10, 0))    -- r_139 = r_10
669    OpMove(ABC(140, 11, 0))    -- r_140 = r_11
670    OpMove(ABC(141, 9, 0))    -- r_141 = r_9
671    OpMove(ABC(142, 10, 0))    -- r_142 = r_10
672    OpMove(ABC(143, 11, 0))    -- r_143 = r_11
673    OpMove(ABC(144, 9, 0))    -- r_144 = r_9
674    OpMove(ABC(145, 10, 0))    -- r_145 = r_10
675    OpMove(ABC(146, 11, 0))    -- r_146 = r_11
676    OpMove(ABC(147, 9, 0))    -- r_147 = r_9
677    OpMove(ABC(148, 9, 0))    -- r_148 = r_9
678    OpMove(ABC(149, 10, 0))    -- r_149 = r_10
679    OpMove(ABC(150, 11, 0))    -- r_150 = r_11
680    OpMove(ABC(151, 9, 0))    -- r_151 = r_9
681    OpMove(ABC(152, 10, 0))    -- r_152 = r_10
682    OpMove(ABC(153, 11, 0))    -- r_153 = r_11
683    OpMove(ABC(154, 9, 0))    -- r_154 = r_9
684    OpMove(ABC(155, 10, 0))    -- r_155 = r_10
685    OpMove(ABC(156, 11, 0))    -- r_156 = r_11
686    OpMove(ABC(157, 9, 0))    -- r_157 = r_9
687    OpMove(ABC(158, 10, 0))    -- r_158 = r_10
688    OpMove(ABC(159, 11, 0))    -- r_159 = r_11
689    OpMove(ABC(160, 9, 0))    -- r_160 = r_9
690    OpMove(ABC(161, 10, 0))    -- r_161 = r_10
691    OpMove(ABC(162, 11, 0))    -- r_162 = r_11
692    OpMove(ABC(163, 9, 0))    -- r_163 = r_9
693    OpMove(ABC(164, 10, 0))    -- r_164 = r_10
694    OpMove(ABC(165, 11, 0))    -- r_165 = r_11
695    OpMove(ABC(166, 9, 0))    -- r_166 = r_9
696    OpMove(ABC(167, 10, 0))    -- r_167 = r_10
697    OpMove(ABC(168, 11, 0))    -- r_168 = r_11
698    OpMove(ABC(169, 9, 0))    -- r_169 = r_9
699    OpMove(ABC(170, 10, 0))    -- r_170 = r_10
700    OpMove(ABC(171, 11, 0))    -- r_171 = r_11
701    OpMove(ABC(172, 9, 0))    -- r_172 = r_9
702    OpUnknown(34)    -- TODO: DESCRIBE OpUnknown(34)
703    OpMove(ABC(123, 9, 0))    -- r_123 = r_9
704    OpMove(ABC(124, 10, 0))    -- r_124 = r_10
705    OpMove(ABC(125, 11, 0))    -- r_125 = r_11
706    OpMove(ABC(126, 9, 0))    -- r_126 = r_9
707    OpMove(ABC(127, 10, 0))    -- r_127 = r_10
708    OpMove(ABC(128, 11, 0))    -- r_128 = r_11
709    OpMove(ABC(129, 9, 0))    -- r_129 = r_9
710    OpMove(ABC(130, 10, 0))    -- r_130 = r_10
711    OpMove(ABC(131, 11, 0))    -- r_131 = r_11
712    OpMove(ABC(132, 9, 0))    -- r_132 = r_9
713    OpMove(ABC(133, 10, 0))    -- r_133 = r_10
714    OpMove(ABC(134, 11, 0))    -- r_134 = r_11
715    OpMove(ABC(135, 9, 0))    -- r_135 = r_9
716    OpMove(ABC(136, 10, 0))    -- r_136 = r_10
717    OpMove(ABC(137, 11, 0))    -- r_137 = r_11
718    OpMove(ABC(138, 9, 0))    -- r_138 = r_9
719    OpMove(ABC(139, 10, 0))    -- r_139 = r_10
720    OpMove(ABC(140, 11, 0))    -- r_140 = r_11
721    OpMove(ABC(141, 9, 0))    -- r_141 = r_9
722    OpMove(ABC(142, 10, 0))    -- r_142 = r_10
723    OpMove(ABC(143, 11, 0))    -- r_143 = r_11
724    OpMove(ABC(144, 9, 0))    -- r_144 = r_9
725    OpMove(ABC(145, 10, 0))    -- r_145 = r_10
726    OpMove(ABC(146, 11, 0))    -- r_146 = r_11
727    OpMove(ABC(147, 9, 0))    -- r_147 = r_9
728    OpUnknown(34)    -- TODO: DESCRIBE OpUnknown(34)
729    OpMove(ABC(121, 122, 0))    -- r_121 = r_122
730    OpMove(ABC(122, 2, 0))    -- r_122 = r_2
731    OpMove(ABC(123, 121, 0))    -- r_123 = r_121
732    OpCall(ABC(122, 2, 4))    -- r_122 .. ... .. r_124 = r_122(r_123 .. ... .. r_123)
733    OpJmp(ASBX(0, 0))    -- goto 734
734    OpUnknown(33)    -- TODO: DESCRIBE OpUnknown(33)
735    OpJmp(ASBX(0, -2))    -- goto 734
736    OpMove(ABC(122, 2, 0))    -- r_122 = r_2
737    OpMove(ABC(123, 121, 0))    -- r_123 = r_121
738    OpCall(ABC(122, 2, 4))    -- r_122 .. ... .. r_124 = r_122(r_123 .. ... .. r_123)
739    OpJmp(ASBX(0, 0))    -- goto 740
740    OpUnknown(33)    -- TODO: DESCRIBE OpUnknown(33)
741    OpJmp(ASBX(0, -2))    -- goto 740
742    OpMove(ABC(122, 2, 0))    -- r_122 = r_2
743    OpMove(ABC(123, 121, 0))    -- r_123 = r_121
744    OpCall(ABC(122, 2, 4))    -- r_122 .. ... .. r_124 = r_122(r_123 .. ... .. r_123)
745    OpJmp(ASBX(0, 0))    -- goto 746
746    OpUnknown(33)    -- TODO: DESCRIBE OpUnknown(33)
747    OpJmp(ASBX(0, -2))    -- goto 746
748    OpMove(ABC(122, 2, 0))    -- r_122 = r_2
749    OpMove(ABC(123, 121, 0))    -- r_123 = r_121
750    OpCall(ABC(122, 2, 4))    -- r_122 .. ... .. r_124 = r_122(r_123 .. ... .. r_123)
751    OpJmp(ASBX(0, 0))    -- goto 752
752    OpUnknown(33)    -- TODO: DESCRIBE OpUnknown(33)
753    OpJmp(ASBX(0, -2))    -- goto 752
754    OpMove(ABC(122, 2, 0))    -- r_122 = r_2
755    OpMove(ABC(123, 121, 0))    -- r_123 = r_121
756    OpCall(ABC(122, 2, 4))    -- r_122 .. ... .. r_124 = r_122(r_123 .. ... .. r_123)
757    OpJmp(ASBX(0, 0))    -- goto 758
758    OpUnknown(33)    -- TODO: DESCRIBE OpUnknown(33)
759    OpJmp(ASBX(0, -2))    -- goto 758
760    OpSetTable(ABC(121, 8, 5))    -- r_121[r_8] = r_5
761    OpGetTable(ABC(122, 121, 8))    -- r_122 = r_121[r_8]
762    OpSetTable(ABC(121, 267, 5))    -- r_121["Index"] = r_5
763    OpGetTable(ABC(123, 121, 267))    -- r_123 = r_121["Index"]
764    OpSetTable(ABC(121, 262, 5))    -- r_121[1] = r_5
765    OpGetTable(ABC(124, 121, 262))    -- r_124 = r_121[1]
766    OpNewTable(ABC(125, 0, 1))    -- r_125 = {} -- 0 list & 1 record
local function proto_9()
    0    OpReturn(ABC(0, 1, 0))    -- return
end
767    OpClosure(ABX(126, 9))    -- TODO: DESCRIBE OpClosure(ABX(126, 9))
768    OpSetTable(ABC(125, 268, 126))    -- r_125["SELFCALL"] = r_126
769    OpMove(ABC(121, 125, 0))    -- r_121 = r_125
770    OpSelf(ABC(125, 121, 268))    -- r_126 = r_121; r_125 = r_121["SELFCALL"]
771    OpCall(ABC(125, 2, 1))    -- r_125(r_126 .. ... .. r_126)
772    OpGetGlobal(ABX(125, 13))    -- r_125 = print
773    OpLoadK(ABX(126, 14))    -- r_126 = "PASSED"
774    OpCall(ABC(125, 2, 1))    -- r_125(r_126 .. ... .. r_126)
775    OpReturn(ABC(0, 1, 0))    -- return
776    OpReturn(ABC(0, 1, 0))    -- return
