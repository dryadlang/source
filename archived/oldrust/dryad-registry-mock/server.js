const express = require('express');
const cors = require('cors');
const bodyParser = require('body-parser');

const app = express();
const PORT = 4000;

app.use(cors());
app.use(bodyParser.json());

// Mock database of packages
// In a real scenario, this would be a database
const packages = {
    "dryad-stdlib": {
        "latest": "0.1.0",
        "versions": {
            "0.1.0": {
                "version": "0.1.0",
                "gitUrl": "C:/Users/Pedro Jesus/Downloads/source-main/source-main/mock-repo",
                "tag": "v0.1.0",
                "hash": "7aea2b515cf13a5d2851c0d102e3e7a1539834581ba99d5adabf71a18f46f1c2",
                "dependencies": {}
            }
        }
    },
    "dryad-utils": {
        "latest": "1.0.0",
        "versions": {
            "1.0.0": {
                "version": "1.0.0",
                "gitUrl": "https://github.com/Dryad-lang/utils.git",
                "tag": "v1.0.0",
                "hash": null,
                "dependencies": {
                    "dryad-stdlib": "^0.1.0"
                }
            }
        }
    },
    // Add more packages as needed
};

// Middleware for logging
app.use((req, res, next) => {
    console.log(`[${new Date().toISOString()}] ${req.method} ${req.url}`);
    next();
});

// Endpoint to getting package info
app.get('/api/packages/:name', (req, res) => {
    const { name } = req.params;
    if (!packages[name]) {
        return res.status(404).json({ error: `Package '${name}' not found` });
    }
    const pkg = packages[name];
    res.json(pkg.versions[pkg.latest]);
});

app.get('/api/packages/:name/:version', (req, res) => {
    const { name, version } = req.params;

    if (!packages[name]) {
        return res.status(404).json({ error: `Package '${name}' not found` });
    }

    const pkg = packages[name];
    if (!pkg.versions[version]) {
        return res.status(404).json({ error: `Version '${version}' for package '${name}' not found` });
    }

    res.json(pkg.versions[version]);
});

// Endpoint to search packages
app.get('/api/search', (req, res) => {
    const { q } = req.query;
    if (!q) {
        return res.json(Object.keys(packages));
    }

    const results = Object.keys(packages).filter(name => name.includes(q));
    res.json(results);
});

app.listen(PORT, () => {
    console.log(`Mock Registry running at http://localhost:${PORT}`);
    console.log(`Try: http://localhost:${PORT}/api/packages/dryad-stdlib/0.1.0`);
});
