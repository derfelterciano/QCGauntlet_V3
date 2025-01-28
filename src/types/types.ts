export interface UserConfig {
	primary_ds: string;
	secondary_ds: string;
	threshold: number;
	controls: ControlDefinitions | null;
	compound_name_col: string;
	well_location_col: string;
	plate_name_col: string;
}

export interface ControlDefinitions {
	control_column: string;
	control_blocks: ControlBlock[];
}

export interface ControlBlock {
	name: string;
	control_wells: string[];
}
