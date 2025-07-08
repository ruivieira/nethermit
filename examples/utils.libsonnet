{
  // Utility functions for array operations
  array: {
    sum(arr):: std.foldl(function(x, y) x + y, arr, 0),
    average(arr):: self.sum(arr) / std.length(arr),
    max(arr):: std.foldl(function(x, y) if y > x then y else x, arr, arr[0]),
    min(arr):: std.foldl(function(x, y) if y < x then y else x, arr, arr[0]),
  },

  // String manipulation utilities
  string: {
    capitalize(str):: std.asciiUpper(str[0]) + str[1:],
    repeat(str, times):: std.join("", std.makeArray(times, function(x) str)),
    truncate(str, length):: if std.length(str) <= length then str else str[0:length] + "...",
  },

  // Object utilities
  object: {
    hasKey(obj, key):: std.objectHas(obj, key),
    getKeys(obj):: std.objectFields(obj),
    getValues(obj):: std.objectValues(obj),
  },
} 