import {defineApplet} from "@akaia/applets-core";
import {SpaceScreen} from "./applet";

export type AkaiaApplet = {
 init: () => void
}

export const AnytypeApplet: AkaiaApplet = defineApplet({
	routes: [{path: "/space", component: SpaceScreen}]
});
