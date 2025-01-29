# **PROJECT ON HOLD (HISTDIFF DESKTOP APP MUST BE MADE FIRST)**

# QCGauntlet_V3

## A Fast Desktop App for Cytological Profiling QC

## **by Derfel Terciano**

Rust/Tauri Desktop version of QC Gauntlet

### Program Description

This python program generates analytical figures that help determine the quality of
experiments. This program can/will generate the following visuals:

_(Note: Some of these features I may see as **not useful** in the future. If so
I will edit this README)_

- Scatter plots of CP Activity Scores (with threshold lines) NOTE: works only with
  two conditions)
- Elbow plots of CP Activity scores (with threshold lines)
- Clustermap files for Java TreeView
- Histograms for control condition correlations
- Barplots of CP Activity control wells that are over a threshold.

**_By default:_** thresholds are set to 0.5

---

### What's different

In this version of QC Gauntlet, I am developing a Tauri-based desktop application that leverages Rust for high-performance calculations and a web-based interface for dynamic and interactive visualizations. Unlike traditional browser-based apps, this Tauri version runs locally on your machine, combining the efficiency of native desktop applications with the modern flexibility of web technologies.

Why use Tauri?

- Lightweight and Fast: Tauri creates minimal binaries with a native feel, ensuring the application runs smoothly even on systems with limited resources.

- Secure and Private: Since all data processing happens locally, your sensitive experimental data remains secure and never leaves your device.

- Interactive Visualizations: The app uses web technologies to provide advanced and intuitive visualizations, enabling users to analyze data in real-time without sacrificing performance.

- Cross-Platform: Build once, and it works seamlessly on Windows, macOS, and Linux.

This approach avoids the need for external hosting or server-side infrastructure, ensuring that QC Gauntlet remains accessible and cost-effective while providing a robust tool for cytological profiling.

---

### User Config

- Datasets:
  - primary_ds: String
  - secondary_ds: Optional string (should be able to handle 1 condition only)
- Meta Information:
  - compound name column
  - well location column
  - plate name column (this is a multiplate processor but should be able to handle single plates as well)
- Threshold: float
- Control Definitions:

  - compound_type column: String (this tells us what is an experimental compound and what is control based)
  - control_group: A list of control definitions that contains the following information:
    - name, list of control wells

- Dataset naming:
  - done through using file name

### TODO

- [x] Implement User Config/Input
- [x] Implement core CP Activity Score Calculations
- [ ] Implement Scatter plot calculations/functionality
