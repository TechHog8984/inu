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
-- NEWTABLE
local e = {}
-- GETTABLE
local f = e[1]
f = e.foo
f = e[c]
-- SETGLOBAL
foo = "bar"

-- CLOSURE
local function proto1()
    -- GETUPVAL
    local a = c
    return a
end

proto1()

print("after proto1")

local function unused_proto()
    proto1()
end
