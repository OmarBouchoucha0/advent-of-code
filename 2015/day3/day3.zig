const std = @import("std");

const Node = struct {
    up: ?*Node,
    down: ?*Node,
    left: ?*Node,
    right: ?*Node,
    visited: bool,
};

fn create_node() Node {
    return Node{
        .up = null,
        .down = null,
        .left = null,
        .right = null,
        .visited = false,
    };
}

fn visitNode(node: *Node) void {
    node.visited = true;
}

fn goUp(from: *Node, to: *Node) void {
    from.up = to;
}

fn goDown(from: *Node, to: *Node) void {
    from.down = to;
}

fn goLeft(from: *Node, to: *Node) void {
    from.left = to;
}

fn goRight(from: *Node, to: *Node) void {
    from.right = to;
}

fn createGraph(input: []const u8, start: *Node) void {
    if (input.len == 0) {
        return;
    }
    // inital starting point
    var curr_node = start;
    for (input) |direction| {
        // this has to also mutate the new destination node because we can go back
        curr_node.visited = true;
        switch (direction) {
            '^' => {
                if (curr_node.up == null) {
                    var destination: Node = create_node();
                    goUp(curr_node, &destination);
                    curr_node = &destination;
                } else {
                    goUp(curr_node, curr_node.up.?);
                    curr_node = curr_node.up.?;
                }
            },
            'v' => {
                if (curr_node.down == null) {
                    var destination: Node = create_node();
                    goDown(curr_node, &destination);
                    curr_node = &destination;
                } else {
                    goDown(curr_node, curr_node.down.?);
                    curr_node = curr_node.down.?;
                }
            },
            '<' => {
                if (curr_node.left == null) {
                    var destination: Node = create_node();
                    goLeft(curr_node, &destination);
                    curr_node = &destination;
                } else {
                    goLeft(curr_node, curr_node.left.?);
                    curr_node = curr_node.left.?;
                }
            },
            '>' => {
                if (curr_node.right == null) {
                    var destination: Node = create_node();
                    goRight(curr_node, &destination);
                    curr_node = &destination;
                } else {
                    goRight(curr_node, curr_node.right.?);
                    curr_node = curr_node.right.?;
                }
            },
            else => {
                unreachable;
            },
        }
    }
}

fn matrix_sol(input: []const u8) !usize {
    if (input.len == 0) {
        return 0;
    }

    var gpa = std.heap.DebugAllocator(.{}){};
    const allocator = gpa.allocator();
    defer _ = gpa.deinit();

    const rows: usize = 1000;
    const cols: usize = 1000;

    var matrix: []u32 = try allocator.alloc(u32, rows * cols);
    defer allocator.free(matrix);

    @memset(matrix, 0);
    var i_santa: usize = @divTrunc(rows, 2);
    var j_santa: usize = @divTrunc(cols, 2);
    var i_bot: usize = @divTrunc(rows, 2);
    var j_bot: usize = @divTrunc(cols, 2);
    var visited: usize = 0;
    var index = (i_santa * cols) + j_santa;
    matrix[index] = 1;
    visited += 1;

    for (input, 0..) |direction, iteration| {
        if (iteration % 2 == 0) {
            switch (direction) {
                '>' => {
                    j_santa += 1;
                },
                '<' => {
                    j_santa -= 1;
                },
                '^' => {
                    i_santa -= 1;
                },
                'v' => {
                    i_santa += 1;
                },
                else => {
                    unreachable;
                },
            }
            index = (i_santa * cols) + j_santa;
            if (matrix[index] == 0) {
                matrix[index] = 1;
                visited += 1;
            }
        } else {
            switch (direction) {
                '>' => {
                    j_bot += 1;
                },
                '<' => {
                    j_bot -= 1;
                },
                '^' => {
                    i_bot -= 1;
                },
                'v' => {
                    i_bot += 1;
                },
                else => {
                    unreachable;
                },
            }
            index = (i_bot * cols) + j_bot;
            if (matrix[index] == 0) {
                matrix[index] = 1;
                visited += 1;
            }
        }
    }
    return visited;
}

pub fn main(init: std.process.Init) !void {
    const io = init.io;
    var buffer: [10240]u8 = undefined;
    const contents = try std.Io.Dir.readFile(std.Io.Dir.cwd(), io, "input.txt", &buffer);
    const instructions = std.mem.trim(u8, contents, " \t\r\n");

    const total_visited = try matrix_sol(instructions);
    std.debug.print("visited {}\n", .{total_visited});
}

// test "empty graph" {
//     const input: []u8 = "";
//     var start = create_node();
//     createGraph(input, &start);
//     try std.testing.expect(start.right == null);
//     try std.testing.expect(start.down == null);
//     try std.testing.expect(start.left == null);
//     try std.testing.expect(start.up == null);
// }
//
// test "graph of 1" {
//     const input = ">";
//     var start = create_node();
//     createGraph(input, &start);
//     try std.testing.expect(start.right != null);
// }
//
// test "graph of 3" {
//     const input = ">^<";
//     var start = create_node();
//     createGraph(input, &start);
//     try std.testing.expect(start.right != null);
// }
test "matrix test empty" {
    const input = "";
    const res = try matrix_sol(input);
    try std.testing.expect(res == 0);
}

test "matrix test 1" {
    const input = ">";
    const res = try matrix_sol(input);
    try std.testing.expect(res == 2);
}

test "matrix test 2" {
    const input = "><";
    const res = try matrix_sol(input);
    try std.testing.expect(res == 3);
}

test "matrix test many" {
    const input = "^>v<";
    const res = try matrix_sol(input);
    try std.testing.expect(res == 3);
}

test "matrix sanat_bot" {
    const input = "^v^v^v^v^v";
    const res = try matrix_sol(input);
    try std.testing.expect(res == 11);
}
