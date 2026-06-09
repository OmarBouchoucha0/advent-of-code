const Prism = struct {
    length: u16,
    width: u16,
    height: u16,
    pub fn surface(self: Prism) u64 {
        return 2 * self.length * self.width + 2 * self.width * self.height + 2 * self.height * self.length;
    }

    pub fn smallest_side(self: Prism) u16 {
        const sides = [3]u16{ self.height * self.width, self.height * self.length, self.width * self.length };
        var min = sides[0];
        for (sides) |side| {
            if (side < min) {
                min = side;
            }
        }
        return min;
    }

    pub fn smallest_perimeter(self: Prism) u32 {
        const sides = [3]u16{ self.height, self.length, self.width };
        var max = sides[0];
        for (sides) |side| {
            if (side > max) {
                max = side;
            }
        }
        return self.height * 2 + self.length * 2 + self.width * 2 - max * 2;
    }
};

fn wrapping_papaer_surface(box: Prism) u64 {
    return box.surface() + box.smallest_side();
}

fn ribbon_length(box: Prism) u64 {
    const bow = box.height * box.length * box.width;
    return bow + box.smallest_perimeter();
}

fn parse_input(allocator: std.mem.Allocator, lines: std.mem.SplitIterator(u8, .sequence)) ![]Prism {
    var list = std.ArrayList(Prism).empty;
    errdefer list.deinit(allocator);

    var iter = lines;
    while (iter.next()) |line| {
        if (line.len == 0) continue;

        var parts = std.mem.splitScalar(u8, line, 'x');

        const l = try std.fmt.parseInt(u16, parts.next() orelse "", 10);
        const w = try std.fmt.parseInt(u16, parts.next() orelse "", 10);
        const h = try std.fmt.parseInt(u16, parts.next() orelse "", 10);

        try list.append(allocator, Prism{ .length = l, .width = w, .height = h });
    }

    return list.toOwnedSlice(allocator);
}

pub fn main(init: std.process.Init) !void {
    const allocator = init.gpa;

    const io = init.io;
    var buffer: [10240]u8 = undefined;
    const contents = try std.Io.Dir.readFile(std.Io.Dir.cwd(), io, "input.txt", &buffer);
    const trimmed = std.mem.trim(u8, contents, " \t\r\n");
    const lines = std.mem.splitSequence(u8, trimmed, "\n");

    const prisms = try parse_input(allocator, lines);
    defer allocator.free(prisms);

    var wrapping_paper_surface_totale: u64 = 0;
    var ribbon_length_total: u64 = 0;
    for (prisms) |box| {
        wrapping_paper_surface_totale += wrapping_papaer_surface(box);
        ribbon_length_total += ribbon_length(box);
    }

    std.debug.print("Totale ribbon needed {}\n", .{ribbon_length_total});
    std.debug.print("Total paper needed: {}\n", .{wrapping_paper_surface_totale});
}

const std = @import("std");
