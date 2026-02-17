# IDM Helper
A native Rust desktop application for Windows to solve a painful SAP IDM role comparison workflow that should have been solved years ago.
This started as a single-file HTML tool. That version worked well and and saved a lot of time, but the nature of a portable document makes the tool feel too constrained.

---

## What Problems This Solves
After our move to SAP S4/HANA, role counts exploded.
Users went from having tens (sometimes 1 or 2 hundred) roles, to having hundreds and even over 1000 in some cases. Comparing access between users was already a slow, manual process, but this change became crippling.

### Pre-Tool Process
- Open IDM role table
- Highlight roles 10-at-a-time (IDM does not load more than 10 roles at a time)
- Copy / Paste roles and useless data into Excel
- Remove useless data
- Remove role prefixes
- Repeat for additional users (comparing access)
- Compare roles between users to find missing access
- Open an Excel file to search for the approver for a role.
    - Many roles have have per-department approvers or other criteria
- Get approval for each role
- Add each roll in IDM one-at-a time.

A single ticket / request can take several hours. Requests involving high-access users can take a day or two

This application exists to cut this down to a few minutes.

---

## What I Built Before This
This project started as a single '.html' tool. It has been reliable and has been a massive productivity booster, even in it's incomplete form.
The tool allows us to:
- Copy a JavaScript web scraping script and paste it into a browser console
- Import role dumps
- Compare access between users
- Format roles in a way approvers can understand
- Export comparison results

I worked with a contractor assisting my company with SAP IDM to get us access to upload results to a directory on a server. The project exports results to a formatted '.txt' file.

The project took me around 1.5 hours to make the initial version, and a few hours to fine-tune it to handle SAP IDM odd-ness after it had proven it's usefullness. But it lacks alot of useful features.
- No approver mapping
- Data Handling isn't safe and can be unpredictable for edge-cases
- Team members can modify the HTML file and break it.

## Why Rebuild as a Native Rust App
### 1) Role Approver Data (SQLite)
The original project was intentionally simple: import files, compare roles, export results.
It did not maintain or reference an internal data store for approvers, roles, and their relationships.
For this version, I need persistent, structured data that can be edited safely through the UI.
Could I do this with JSON? Yes.
Do I want non-technical users touching raw JSON files for hundreds of role mappings? No

SQLite gives me:
- Schema constraints
- Safer Updates
- Cleaner relationships
- Less chance of "Somebody opened the file and accidentally pressed a key on their keyboard and now everything is broken"

Users can edit data through the UI. The app handles structure and validation

### 2) Rust is Perfect for This Type of Project
This app is mostly:
- Parsing text or csv files
- Transforming role identifiers
- Filtering + comparing large data sets
- Exporting reliable output

Rust:
- Is type safe
- Explicit error handling
- Great performance
- Low resource usage
- Simple distribution

**Bonus**: I already know Rust well enough to ship this without making it an abandoned experiment.

### 3) Protable HTML is Convenient, But too Easy to Mutate
Sending an editable HTML file over Teams is really convenient, but a single 2k LoC file is hard to work in when trying to add features.
Depending on an editable HTML file also bears the risk of the "master" version being accidentally (or maliciously edited).

### 4) Most of My Team Does Not Work With SAP IDM Role Management
Most of my team has not been trained in SAP role management. Part of the goal of this application is to help lighten the load off of me and the one other person that has experience. This application does most of the work and removes the need to train less-technical users, allowing us to work on the dozens of other responsibilities we have.

### 5) No Support Offered
I do not work with Software Developers, and I am not a Software Developer. I am a guy who spends most of his free-time coding *for the love of the game*. Once this project is done I will not support or maintain it. We cannot have a file that can be easily broken. This is a project that I have built mostly in my own time. This particular version was built entirely in my own time. It needs to be rock solid and the logic needs to be immutable until IDM is phased out from my company.

---

## Tech Stack
- **Language:** Rust
- **GUI:** egui + eframe (native desktop, not browser)
- **Storage:** SQLite

---

## Current Direction
Planned module layout:
- `core/` for parsing, filtering, trasforms, comparison, export
- `state/` for app state + domain models
- `ui/` for panels/widgets (settings, users, results, script instructions, etc)

---

## Tradeoffs I'm Making
- I'm choosing native desktop over portability
- I'm choosing schema/data integrity over a "just edit a text file" workflow
- I'm choosing to rebuild this for stability, performance, and to implement features that would be too much to do in a single HTML file.

---

## What This Is / What This Is Not

**This is:**
- a practical internal productivity tool
- built to remove manual, repetitive access-comparison work
- a real-world Rust GUI project for my portfolio

**This is not:**
- a polished enterprise product
- a generic IDM framework
- tied to any identifiable internal company data in this repo

---

## Why I Built It
The problem was rediculous and was not ever acknowledged. I know how to solve it, so I did. Spending hours - days on a single ticket is absurd for something so trivial.

---

Much of the original HTML project was done on my own time.
This Rust version is entirely built on my own, off-the-clock time on my own hardware from my car (aka home).