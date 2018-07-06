__kernel void next_generation(__global const uchar *world, __global uchar *next_world) {
  size_t x = get_global_id(0);
  size_t y = get_global_id(1);
  size_t w = get_global_size(0);
  size_t h = get_global_size(1);

  bool is_left   = x > 0  ;
  bool is_right  = x < w-1;
  bool is_top    = y > 0  ;
  bool is_bottom = y < h-1;

  size_t n
    = (is_top                ? world[ x      + (y - 1)*w] : 0) // top
    + (is_right && is_top    ? world[(x + 1) + (y - 1)*w] : 0) // top right
    + (is_right              ? world[(x + 1) +  y     *w] : 0) // right
    + (is_right && is_bottom ? world[(x + 1) + (y + 1)*w] : 0) // bottom right
    + (is_bottom             ? world[ x      + (y + 1)*w] : 0) // bottom
    + (is_left  && is_bottom ? world[(x - 1) + (y + 1)*w] : 0) // bottom left
    + (is_left               ? world[(x - 1) +  y     *w] : 0) // left
    + (is_left  && is_top    ? world[(x - 1) + (y - 1)*w] : 0) // top left
    ;

  size_t index = x + y*w;
  next_world[index] = world[index] ? (n == 2 || n == 3) : (n == 3);
}
