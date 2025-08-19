# Restore Sites

A modern, cross-platform desktop application for managing and restoring website collections. Built with Tauri, SvelteKit, and Rust.

![Restore Sites Screenshot](docs/screenshot.png)

## Overview

Restore Sites is a GUI version of the original restore.py script, providing a simple and lightweight interface for saving website collections and restoring them in your browser. Perfect for managing research sessions, work projects, or any set of websites you frequently visit together.

## Features

### ✅ Core Functionality
- **Save Website Collections**: Parse title-URL pairs from pasted text
- **Browser Integration**: Open collections in your preferred browser
- **Cross-Platform**: Works on macOS, Windows, and Linux
- **Dark Mode**: Full dark/light theme support
- **Lightweight Storage**: JSON-based persistence at `~/.restore-sites/db.json`

### ✅ Browser Profiles System
- **Reusable Configurations**: Create browser profiles with custom settings
- **Auto-Detection**: Visual indicators showing which browsers are installed
- **Smart Fallbacks**: Profile → Custom → Global → System default chain
- **Multiple Modes**: Normal, Incognito, and Private browsing support
- **Custom Paths**: Support for non-standard browser installations

### ✅ Advanced Collection Management
- **Individual Site Editing**: Edit titles and URLs inline
- **Selective Restoration**: Choose which sites to open with checkboxes
- **Recent Collections**: Quick access to recently updated collections
- **Collection Configuration**: Per-collection browser settings
- **Full Type Safety**: TypeScript/Rust type synchronization

## Technology Stack

- **Frontend**: SvelteKit 5, TypeScript, Tailwind CSS 4
- **Backend**: Tauri 2, Rust
- **Database**: JSON file storage
- **Package Manager**: Bun
- **UI Components**: Lucide icons, svelte-sonner toasts

## Installation

### Download Pre-built Binaries
*(Coming soon - check Releases page)*

### Build from Source

#### Prerequisites
- [Rust](https://rustup.rs/) (latest stable)
- [Bun](https://bun.sh/) (package manager)
- [Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites/)

#### Build Steps
```bash
# Clone the repository
git clone https://github.com/your-username/restore-sites-gui.git
cd restore-sites-gui

# Install frontend dependencies
bun install

# Run in development mode
bun run tauri dev

# Build for production
bun run tauri build
```

## Usage

### Quick Start
1. Launch Restore Sites
2. Paste your browser tabs in the format: `Title https://example.com`
3. Click "Save Collection" 
4. View and manage your collections in the Collections page
5. Click "Restore All" to open all sites in your browser

### Text Input Format
```
My Project Dashboard https://dashboard.example.com
GitHub Repository https://github.com/user/repo
Documentation Site https://docs.example.com/guide
```

### Browser Profiles
1. Go to Settings → Browser Profiles
2. Click "Create Profile" to set up reusable browser configurations
3. Configure browser, mode (Normal/Incognito/Private), and custom paths
4. Assign profiles to collections for consistent opening behavior

### Environment Variables
- `RUST_LOG`: Control logging level (debug, info, warn, error)
  - Development default: `debug,info`
  - Production default: `warn`

## Development

### Project Structure
```
src/
├── lib/
│   ├── api/              # Tauri API wrappers
│   ├── types/           # TypeScript type definitions
│   └── utils/           # Utility functions
├── routes/              # SvelteKit pages
│   ├── collections/     # Collection management
│   └── settings/        # Browser profiles & settings
├── components/          # Reusable UI components
└── app.css             # Global styles

src-tauri/src/
├── commands.rs         # Tauri command handlers
├── database.rs         # JSON storage operations
├── models.rs          # Rust data structures
├── services.rs        # Business logic
└── utils.rs           # Helper functions
```

### Development Commands
```bash
# Start development server
bun run dev                 # Frontend only (port 1420)
bun run tauri dev          # Full Tauri development

# Type checking
bun run check              # One-time check
bun run check:watch       # Watch mode

# Building
bun run build             # Production build
bun run tauri build      # Create distributable
```

### Database Schema
Collections are stored in `~/.restore-sites/db.json`:
```json
{
  "meta": {
    "version": 2,
    "default_browser_mode": "Normal",
    "max_id": 123,
    "record_count": 57
  },
  "profiles": [
    {
      "id": "default-chrome",
      "name": "Default Chrome",
      "browser": "Chrome",
      "mode": "Incognito",
      "is_default": true,
      "is_detected": true
    }
  ],
  "data": [
    {
      "id": 1,
      "name": "My Project",
      "sites": [
        {"title": "GitHub", "url": "https://github.com"}
      ],
      "config": {
        "browser_profile_id": "default-chrome"
      }
    }
  ]
}
```

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes and test thoroughly
4. Commit with clear messages: `git commit -m "Add feature description"`
5. Push to your fork: `git push origin feature-name`
6. Create a Pull Request

### Code Style
- **Rust**: Follow `rustfmt` formatting
- **TypeScript**: ESLint + Prettier configuration
- **Svelte**: Use Svelte 5 runes syntax (`$state`, `$props`, `$derived`)
- **Testing**: Ensure new features have appropriate tests

## Architecture Decisions

### Browser Profiles Design
- **Single JSON storage** for simplicity and performance
- **4-tier resolution** for flexible configuration inheritance
- **Manual detection updates** with visual status indicators
- **UTF-8 safe naming** with 64 character limits

### Type Safety
- **Rust structs** have exact TypeScript mirrors
- **API wrappers** provide compile-time safety
- **Breaking changes** in Rust break TypeScript compilation (by design)

## Roadmap

### Planned Features
- Collection search and filtering
- Bulk collection operations
- Import/export functionality
- Profile templates and presets
- Collection tagging and organization
- Keyboard shortcuts
- Collection sharing

### Known Limitations
- Single JSON file storage (8MiB practical limit)
- Manual browser detection updates
- No real-time collaboration features

## License

[MIT License](LICENSE) - see LICENSE file for details.

## Acknowledgments

- Inspired by the original restore.py Python script
- Built with the amazing [Tauri](https://tauri.app/) framework
- UI designed with [Tailwind CSS](https://tailwindcss.com/)
- Icons provided by [Lucide](https://lucide.dev/)

---

**Made with ❤️ for productivity and organization**
