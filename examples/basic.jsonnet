{
  // Local variables
  local greeting = "Hello",
  local name = "World",
  local numbers = {
    a: 10,
    b: 20,
  },

  // Basic arithmetic and string concatenation
  message: greeting + " " + name + "!",
  sum: numbers.a + numbers.b,
  product: numbers.a * numbers.b,
  
  // Conditional expression
  isPositive: if numbers.a > 0 then "Yes" else "No",

  // Array comprehension
  squares: [x * x for x in [1, 2, 3, 4, 5]],

  // Object with computed field names
  computed: {
    value: numbers.a,
    doubled: numbers.a * 2,
  },
} 