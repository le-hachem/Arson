<p align="center">
  <img src="https://raw.githubusercontent.com/le-hachem/Arson/refs/heads/main/src-tauri/icons/128x128%402x.png" alt="Logo" width="200"/>
</p>

# Arson
**Arson** is an open-source desktop and web application built with [Tauri](https://tauri.app/) and [Yew](https://yew.rs/), designed to visualize conflict event data from the [Armed Conflict Location & Event Data Project (ACLED)](https://acleddata.com/) on an interactive, zoomable map powered by [Leaflet](https://leafletjs.com/).

---

## Features
* **Live Visualization:** Displays ACLED conflict event data in real-time on an intuitive, interactive map
* **User-Controlled Access:** Fetches data directly through the official ACLED API using **your own API key**
* **No Data Storage:** Does **not** store, cache, or redistribute any ACLED data—your API key controls all data access
* **Open Source & Transparent:** Fully open codebase so you can review exactly how data is accessed and visualized
* **Cross-Platform:** Runs on Windows, macOS, and Linux as a desktop app via Tauri, and also supports web deployment

---

## API Key and Data Usage
* **You must provide your own valid ACLED API key** to use this application.
* The app makes direct API calls to ACLED servers using your key, so **all data permissions and restrictions are enforced by ACLED’s API** based on your key’s license.
* Arson does **not** store, cache, or share ACLED data independently; your API key governs what data you can access and view.
* Please ensure you comply with [ACLED’s data license and terms of service](https://acleddata.com/data/using-the-data/license/).

---

## License
This project is licensed under the **GNU Affero General Public License v3.0 (AGPL-3.0)**.

* You are free to use, modify, and distribute this software under the terms of the AGPL-3.0.
* If you modify or deploy the app as a network service, you must make your modified source code publicly available under the same license.
* The use of ACLED data through this app is subject to ACLED’s licensing terms and conditions and remains your responsibility as the API key holder.

See the [LICENSE](./LICENSE) file for full details.
