[
  // First document: User profiles
  {
    type: "profiles",
    users: [
      {
        name: "Alice",
        role: "admin",
        preferences: {
          theme: "dark",
          notifications: true,
        },
      },
      {
        name: "Bob",
        role: "user",
        preferences: {
          theme: "light",
          notifications: false,
        },
      },
    ],
  },

  // Second document: System settings
  {
    type: "settings",
    database: {
      host: "localhost",
      port: 5432,
      maxConnections: 100,
    },
    cache: {
      enabled: true,
      ttl: 3600,
    },
  },

  // Third document: Feature flags
  {
    type: "features",
    flags: {
      darkMode: true,
      betaFeatures: false,
      analytics: {
        enabled: true,
        samplingRate: 0.1,
      },
    },
    rolloutPercentage: 50,
  },
] 