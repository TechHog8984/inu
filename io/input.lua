-- https://gist.github.com/TheGreatSageEqualToHeaven/35391fa59a37e89898232650c2290392
-- load

local math = math                     -- GETGLOBAL
local newproxy = newproxy             -- GETGLOBAL
local ipairs = ipairs                 -- GETGLOBAL
local floor = math.floor              -- GETTABLE KST(C)
local pi = math.pi                    -- GETTABLE KST(C)
local _nil = nil                      -- LOADNIL B -> C (1)
local _true = true                    -- LOADBOOL B(1)
local _false = false                  -- LOADBOOL B(0)
local _kst = "Hello World"            -- LOADK
local _1 = 1                          -- LOADK
local _2 = 2                          -- LOADK
local _3 = 3                          -- LOADK

local a, b, c, d, e, f, g, h, i, j, k -- LOADNIL B -> C (10)

NEW_GLOBAL = gbl;                     -- GETGLOBAL SETGLOBAL

local ds = pi + pi                    -- ADD
local ds = pi + 1                     -- ADD KST(C)
local ds = 1 + pi                     -- ADD KST(B)
local ds = pi - pi                    -- SUB
local ds = pi - 1                     -- SUB KST(C)
local ds = 1 - pi                     -- SUB KST(B)
local ds = pi * pi                    -- MUL
local ds = pi * 1                     -- MUL KST(C)
local ds = 1 * pi                     -- MUL KST(B)
local ds = pi / pi                    -- DIV
local ds = pi / 1                     -- DIV KST(C)
local ds = 1 / pi                     -- DIV KST(B)
local ds = pi ^ pi                    -- POW
local ds = pi ^ 1                     -- POW KST(C)
local ds = 1 ^ pi                     -- POW KST(B)
local ds = pi % pi                    -- MOD
local ds = pi % 1                     -- MOD KST(C)
local ds = 1 % pi                     -- MOD KST(B)

-- [[ MOVE's ]] --

local ds = _kst .. _kst                 -- CONCAT B -> C (1)
local ds = _kst .. _kst .. _kst         -- B -> C (2)
local ds = _kst .. _kst .. _kst .. _kst -- B -> C (3)

local ds = -_1;                         -- UNM
local ds = not _false;                  -- NOT
local ds = #_kst;                       -- LEN

-- cf

-- [[ LOADK / MOVE ]] --
for ds = _1, _2 do -- sbx(-1)
end

for ds = _1, _2, _3 do -- sbx(-1)
end

for ds = _1, _2, _3 do -- sbx(-2)
    local ds           -- LOADNIL B -> C (0)
end

while _false do -- TEST C(0) JMP sbx(1) sbx(-3)
end

while _false do -- TEST C(0) JMP sbx(1) sbx(-4)
    local ds    -- LOADNIL B -> C (0)
end

while not _true do -- TEST C(1) JMP sbx(1) sbx(-4)
end

while not _true do -- TEST C(1) JMP sbx(1) sbx(-4)
    local ds       -- LOADNIL B -> C (0)
end

repeat
until _true -- TEST C(0) JMP sbx(-2)

repeat
    local ds -- LOADNIL B -> C (0)
until _true  -- TEST C(0) JMP sbx(-3)

repeat
until not _false -- TEST C(1) JMP sbx(-2)

repeat
    local ds          -- LOADNIL B -> C (0)
until not _false      -- TEST C(1) JMP sbx(-3)

local ds = b or c     -- TESTSET C(1)
local ds = not b or c -- TESTSET C(0)

-- [[ JMP sBx(1) LOADBOOL LOADBOOL ]] --

local ds = pi == 1;  -- EQ A(1) KST(C)
local ds = 1 == pi;  -- EQ A(1) KST(B)
local ds = pi == pi; -- EQ A(1) EQ
local ds = 1 == 1;   -- EQ A(1) KST(B) KST(C)

local ds = pi ~= 1;  -- EQ A(0) KST(C)
local ds = 1 ~= pi;  -- EQ A(0) KST(B)
local ds = pi ~= pi; -- EQ A(0) EQ
local ds = 1 ~= 1;   -- EQ A(0) KST(B) KST(C)

local ds = pi <= 1;  -- EQ A(1) KST(C)
local ds = 1 <= pi;  -- EQ A(1) KST(B)
local ds = pi <= pi; -- EQ A(1) EQ
local ds = 1 <= 1;   -- EQ A(1) KST(B) KST(C)

local ds = pi >= 1;  -- EQ A(1) KST(C)
local ds = 1 >= pi;  -- EQ A(1) KST(B)
local ds = pi >= pi; -- EQ A(1) EQ
local ds = 1 >= 1;   -- EQ A(1) KST(B) KST(C)

-- [[ MOVE ]] --

newproxy()                 -- CALL B(1) C(1)
local a = newproxy()       -- CALL B(1) C(2)
local a, b = newproxy()    -- CALL B(1) C(3)
local a, b, c = newproxy() -- CALL B(1) B(4)

-- [[ MOVE MOVE ]] --

floor(pi)                 -- CALL B(2) C(1)
local a = floor(pi)       -- CALL B(2) C(2)
local a, b = floor(pi)    -- CALL B(2) C(3)
local a, b, c = floor(pi) -- CALL B(2) C(4)

-- [[ MOVE MOVE MOVE ]] --

newproxy(_nil, _nil)                 -- CALL B(3) C(1)
local a = newproxy(_nil, _nil)       -- CALL B(3) C(2)
local a, b = newproxy(_nil, _nil)    -- CALL B(3) C(3)
local a, b, c = newproxy(_nil, _nil) -- CALL B(3) C(4)

local function b1()                  -- no upvalues
    return n                         -- RETURN B(2)
end

local function b2() -- GETUPVAL
    return _kst     -- RETURN B(2)
end

local function b2m()       -- GETUPVAL
    _kst = "Goodbye World" -- SETUPVAL
    return _kst            -- RETURN B(2)
end

local function b3() -- GETUPVAL GETUPVAL
    return _1, _2   -- RETURN B(3)
end

local function b3m()  -- GETUPVAL GETUPVAL GETUPVAL
    return _1, _2, _3 -- RETURN B(4)
end

local function b1e()
    return _1, _1, _1, _1, _1, _1, _1, _1, _1, _1, _1, _1, _1, _1, _1, _1, _1, _1, _1 -- RETURN B(19)
end

local function tco(...) -- VARARG
    return b3(...)      -- TCO B(1) C = MULTI_RET
end

local function tco1()
    return b3(_nil) -- TCO B(2) C = MULTI_RET
end

local function tco2()
    return b3(_nil, _nil) -- TCO B(3) C = MULTI_RET
end

b1()                                                                                                -- CALL B(1) C(1)
local a = b2()                                                                                      -- CALL B(1) C(2)
b2m()                                                                                               -- CALL B(1) C(1)
local a, b = b3()                                                                                   -- CALL B(1) C(3)
local a, b, c = b3m()                                                                               -- CALL B(1) C(4)
local a = tco()                                                                                     -- CALL B(1) C(2)
local a, b = tco1()                                                                                 -- CALL B(1) C(3)
local a, bc = tco2()                                                                                -- CALL B(1) C(4)

local a, b, c, d, e, f, g, h, l = b1e()                                                             -- CALL B(1) C(9)
b1e(_nil, _nil, _nil, _nil, _nil, _nil, _nil, _nil, _nil, _nil, _nil, _nil, _nil, _nil, _nil, _nil) -- CALL B(16) C(1)
local a, b, c, d, e, f, g, h, l = b1e()                                                             -- CALL B(1) C(9)
b1e(_nil, _nil, _nil, _nil, _nil, _nil, _nil, _nil, _nil, _nil, _nil, _nil, _nil, _nil, _nil, _nil) -- CALL B(16) C(1)

-- tables
local _table = {} -- NEWTABLE
_table = {        -- SETLIST B(3) C(1)
    _1, _2, _3
}
_table = { _1 } -- SETLIST B(1) C(1)
_table = {      -- SETLIST(B50) C(1)
    _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1,
    _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1
}
_table = { -- SETLIST(B50) C(1) SETLIST B(50) (1)
    _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1,
    _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1
, _1
}
_table = { -- SETLIST B(50) C(1) SETLIST B(50) C(1)
    _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1,
    _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1,
    _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1,
    _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1
}
_table = { -- SETLIST B(50) C(2 -> 4 | Encoded Block)
    _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1,
    _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1,
    _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1,
    _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1,
    _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1,
    _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1,
    _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1,
    _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1,
    _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1, _2, _3, _1,
}

for i in ipairs(_table) do             -- CALL TFORLOOP B(1)
end
for i, v in ipairs(_table) do          -- CALL TFORLOOP B(2)
end
for i, v, k in ipairs(_table) do       -- CALL TFORLOOP B(3)
end
for i, v, k, e in ipairs(_table) do    -- CALL TFORLOOP B(2)
end
for i, v, k, e, f in ipairs(_table) do -- CALL TFORLOOP B(2)
end

_table[_kst] = _nil                        -- SETTABLE
local ds = _table[_kst]                    -- GETTABLE
_table["Index"] = _nil                     -- SETTABLE KST(B)
local ds = _table["Index"]                 -- GETTABLE KST(C)
_table[1] = _nil                           -- SETTABLE KST(B)N
local ds = _table[1]                       -- GETTABLE KST(C)N

_table = { ["SELFCALL"] = function() end } -- NEWTABLE CLOSURE SETTABLE
_table:SELFCALL()                          -- SELF

print("PASSED")
return -- RETURN B(1) RETURN B(1)
