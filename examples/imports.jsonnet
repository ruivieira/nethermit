local utils = import './utils.libsonnet';

{
  // Using array utilities
  numbers: {
    data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    sum: utils.array.sum(self.data),
    average: utils.array.average(self.data),
    max: utils.array.max(self.data),
    min: utils.array.min(self.data),
  },

  // Using string utilities
  strings: {
    local text = "hello world",
    capitalized: utils.string.capitalize(text),
    repeated: utils.string.repeat("*-", 5),
    truncated: utils.string.truncate("This is a very long text that needs to be truncated", 20),
  },

  // Using object utilities
  objects: {
    local testObj = {
      name: "Test",
      value: 42,
      active: true,
    },

    hasNameKey: utils.object.hasKey(testObj, "name"),
    allKeys: utils.object.getKeys(testObj),
    allValues: utils.object.getValues(testObj),
  },

  // Combining multiple utilities
  combined: {
    local data = {
      scores: [85, 92, 78, 95, 88],
      student: "alice smith",
    },

    averageScore: utils.array.average(data.scores),
    studentName: utils.string.capitalize(data.student),
    summary: "Student " + self.studentName + " has an average score of " + self.averageScore,
  },
} 