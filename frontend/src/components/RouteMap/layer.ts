import type Map from "esri/Map";
import type GraphicsLayer from "esri/layers/GraphicsLayer";

abstract class Layer {
    readonly layer: GraphicsLayer;

    protected constructor(map: Map, layer: GraphicsLayer) {
        this.layer = layer;
        map.layers.add(this.layer);
    }

    clear() {
        this.layer.graphics.removeAll();
    }
}

export default Layer;