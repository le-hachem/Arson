use crate::components::{generate_popup_content, AcledEvent};
use crate::logging::console;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = L)]
    type LeafletMap;

    #[wasm_bindgen(js_namespace = L, js_name = map)]
    fn leaflet_map(id: &str, options: &JsValue) -> LeafletMap;

    #[wasm_bindgen(js_namespace = L, method)]
    fn setView(this: &LeafletMap, latlng: &JsValue, zoom: f64) -> LeafletMap;

    #[wasm_bindgen(js_namespace = L)]
    type TileLayer;

    #[wasm_bindgen(js_namespace = L, js_name = tileLayer)]
    fn tile_layer(url: &str, options: &JsValue) -> TileLayer;

    #[wasm_bindgen(js_namespace = L, method)]
    fn addTo(this: &TileLayer, map: &LeafletMap) -> TileLayer;

    #[wasm_bindgen(js_namespace = L)]
    type Marker;

    #[wasm_bindgen(js_namespace = L, js_name = marker)]
    fn leaflet_marker(latlng: &JsValue, options: &JsValue) -> Marker;

    #[wasm_bindgen(js_namespace = L, method)]
    fn addTo(this: &Marker, map: &LeafletMap) -> Marker;

    #[wasm_bindgen(js_namespace = L, method)]
    fn bindPopup(this: &Marker, content: &str) -> Marker;

    #[wasm_bindgen(js_namespace = L)]
    type LayerGroup;

    #[wasm_bindgen(js_namespace = L, js_name = layerGroup)]
    fn layer_group() -> LayerGroup;

    #[wasm_bindgen(js_namespace = L, method)]
    fn addLayer(this: &LayerGroup, layer: &Marker) -> LayerGroup;

    #[wasm_bindgen(js_namespace = L, method)]
    fn addTo(this: &LayerGroup, map: &LeafletMap) -> LayerGroup;

    #[wasm_bindgen(js_namespace = L, method)]
    fn clearLayers(this: &LayerGroup) -> LayerGroup;
}

#[derive(Properties, PartialEq)]
pub struct MapDisplayProps {
    pub events: Option<Vec<AcledEvent>>,
}

#[function_component(MapDisplay)]
pub fn map_display(props: &MapDisplayProps) -> Html {
    let map_ref = use_node_ref();
    let map_instance = use_state(|| Option::<LeafletMap>::None);
    let markers_layer = use_state(|| Option::<LayerGroup>::None);

    // Initialize map
    {
        let map_ref = map_ref.clone();
        let map_instance = map_instance.clone();

        use_effect_with((), move |_| {
            if let Some(map_element) = map_ref.cast::<HtmlElement>() {
                map_element.set_id("leaflet-map");

                let map_options = js_sys::Object::new();
                js_sys::Reflect::set(&map_options, &"zoomControl".into(), &true.into()).unwrap();
                js_sys::Reflect::set(&map_options, &"attributionControl".into(), &true.into())
                    .unwrap();

                let leaflet_map = leaflet_map("leaflet-map", &map_options.into());

                let latlng = js_sys::Array::new();
                latlng.push(&33.8869.into());
                latlng.push(&35.5131.into());
                leaflet_map.setView(&latlng.into(), 7.0);

                let tile_options = js_sys::Object::new();
                js_sys::Reflect::set(
                    &tile_options,
                    &"attribution".into(),
                    &"Tiles &copy; Esri &mdash; Source: Esri, i-cubed, USDA, USGS, AEX, GeoEye, Getmapping, Aerogrid, IGN, IGP, UPR-EGP, and the GIS User Community".into()
                ).unwrap();
                js_sys::Reflect::set(&tile_options, &"maxZoom".into(), &18.into()).unwrap();

                let tile_layer = tile_layer(
                    "https://server.arcgisonline.com/ArcGIS/rest/services/World_Imagery/MapServer/tile/{z}/{y}/{x}",
                    &tile_options.into(),
                );
                tile_layer.addTo(&leaflet_map);

                let markers = layer_group();
                markers.addTo(&leaflet_map);

                map_instance.set(Some(leaflet_map));
                console::log_with_context!(
                    "MAP_DISPLAY",
                    "Leaflet satellite map initialized successfully"
                );
            } else {
                console::error_with_context!("MAP_DISPLAY", "Failed to get map element reference");
            }

            || ()
        });
    }

    // Update markers when events change
    {
        let map_instance = map_instance.clone();
        let markers_layer = markers_layer.clone();
        let events = props.events.clone();

        use_effect_with(events.clone(), move |events| {
            if let (Some(map), Some(events)) = ((*map_instance).as_ref(), events.as_ref()) {
                if let Some(markers) = (*markers_layer).as_ref() {
                    markers.clearLayers();
                } else {
                    let new_markers = layer_group();
                    new_markers.addTo(map);
                    markers_layer.set(Some(new_markers));
                }

                if let Some(markers) = (*markers_layer).as_ref() {
                    for event in events {
                        if let (Some(lat), Some(lng)) = (event.latitude, event.longitude) {
                            let latlng = js_sys::Array::new();
                            latlng.push(&lat.into());
                            latlng.push(&lng.into());

                            let marker_options = js_sys::Object::new();
                            let marker = leaflet_marker(&latlng.into(), &marker_options.into());

                            let popup_content = generate_popup_content(event);
                            marker.bindPopup(&popup_content);
                            markers.addLayer(&marker);
                        }
                    }

                    console::log_with_context!(
                        "MAP_DISPLAY",
                        "Added {} markers to map",
                        events.len()
                    );
                }
            }

            || ()
        });
    }

    html! {
        <>
            <div class="map-content">
                <div ref={map_ref} class="leaflet-map-wrapper">
                </div>
            </div>

            if let Some(events) = &props.events {
                <div class="map-stats">
                    <p><strong>{"Events on Map: "}</strong>{events.len()}</p>
                    <p style="font-size: 0.9em; color: #888888;">{"Click markers for event details"}</p>
                </div>
            }
        </>
    }
}
