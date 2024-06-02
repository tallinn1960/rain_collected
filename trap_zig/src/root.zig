const minInt = @import("std").math.minInt;

pub fn trap(terrain: []const i64) u64 {
    var l: usize = 0;
    var r = terrain.len - 1;
    var countWater: u64 = 0;
    var level: i64 = minInt(i64);
    var tl = terrain[l];
    var tr = terrain[r];

    while (l < r) {
        level = @max(level, @min(tl, tr));
        if (tl < tr) {
            countWater += @intCast(level - tl);
            l += 1;
            tl = terrain[l];
        } else {
            countWater += @intCast(level - tr);
            r -= 1;
            tr = terrain[r];
        }
    }
    return countWater;
}

export fn trap_zig(p: [*]const i64, len: usize) u64 {
    const terrain: []const i64 = p[0..len];
    return trap(terrain);
}

const expect = @import("std").testing.expect;

test "trap" {
    try expect(trap(&[_]i64{ 0, 0, 0, 0, 0 }) == 0);
    try expect(trap(&[_]i64{ 1, 2, 3, 4, 5 }) == 0);
    try expect(trap(&[_]i64{ 5, 4, 3, 2, 1 }) == 0);
    try expect(trap(&[_]i64{ 1, 2, 3, 2, 1 }) == 0);
    try expect(trap(&[_]i64{ 1, 2, 3, 2, 4, 1 }) == 1);
    try expect(trap(&[_]i64{ 1, 4, 2, 5, 3, 6, 4, 7 }) == 6);
    try expect(trap(&[_]i64{ 2, 1, 2 }) == 1);
    try expect(trap(&[_]i64{ 5, 4, 2, 6, 6, 6, 4, 5 }) == 5);
    try expect(trap(&[_]i64{ 0, 1, -1, 2, 1, 0, 1, 3, 2, 1, 2, 1 }) == 7);
    try expect(trap(&[_]i64{ 4, 2, 0, 3, 2, 5 }) == 9);
    try expect(trap(&[_]i64{ 0, -6, 0, -2, 8, -9, 0, 8, 9, -5 }) == 33);
}
