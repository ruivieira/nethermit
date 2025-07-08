{
  // Function to create a person object
  local createPerson(name, age, hobbies=[]) = {
    name: name,
    age: age,
    hobbies: hobbies,
    isAdult: age >= 18,
    description: "My name is " + name + " and I am " + age + " years old.",
  },

  // Function to merge two objects
  local mergeObjects(obj1, obj2) = obj1 + obj2,

  // Create some people
  people: {
    alice: createPerson("Alice", 25, ["reading", "hiking"]),
    bob: createPerson("Bob", 30, ["gaming", "cooking"]),
  },

  // Object manipulation
  settings: {
    local base = {
      theme: "light",
      fontSize: 12,
      language: "en",
    },

    local overrides = {
      theme: "dark",
      fontSize: 14,
    },

    // Merge base settings with overrides
    config: mergeObjects(base, overrides),

    // Create a list of all available themes
    themes: ["light", "dark", "high-contrast"],
  },

  // Function with default parameters
  local formatCurrency(amount, currency="USD", decimals=2) =
    std.toString(std.floor(amount * std.pow(10, decimals)) / std.pow(10, decimals)) + " " + currency,

  // Use the currency formatter
  prices: {
    basic: formatCurrency(9.99),
    premium: formatCurrency(29.99, "EUR"),
    enterprise: formatCurrency(99.99, "GBP", 0),
  },
} 