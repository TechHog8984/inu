-- GETGLOBAL
local a = b
-- MOVE
local c = a
-- LOADK
local b = "\a, \b, \f, \n, \r, \t, \v, \\, \', \", Hello, World!"
-- LOADBOOL
c = true
-- LOADNIL
local d = nil
-- NEWTABLE & SETLIST
local e = { "foo" }
-- GETTABLE
local f = e[1]
f = e.foo
f = e[c]
-- SETTABLE
e.foo = 1
e.bar = e[e.foo]
-- SETGLOBAL
foo = "bar"

-- CLOSURE
function f:foo()
    -- SETUPVAL
    d = 1
    -- GETUPVAL
    return c
end

-- SELF & CALL
f:foo()

local function unused_proto()

end

-- ADD
local a = a + 1
a = 1 + a
-- SUB
local a = a - 1
a = 1 - a
-- MUL
local a = a * 1
a = 1 * a
-- DIV
local a = a / 1
a = 1 / a
-- MOD
local a = a % 1
a = 1 % a
-- POW
local a = a ^ 1
a = 1 ^ a
-- UNM
local a = -a
-- NOT
local a = not a
-- LEN
local a = #f
-- CONCAT
local a = "hello" .. ' ' .. "world"

-- TEST & GOTO
if a then
    print "hi"
end

-- EQ
if a == b then
    print "hi"
end
-- LT
if a < b then
    print "hi"
end
-- LE
if a <= b then
    print "hi"
end
