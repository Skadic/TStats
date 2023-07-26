export type PoolMap = {
	/// The id of the tournament this pool belongs to.
	tournament_id: number;
	/// The index of the stage in the tournament this pool belongs to.
	stage_order: number;
	/// The order of this bracket in the stage. E.g. if this is the first bracket in the pool, this is 0.
	bracket_order: number;
	/// The number of the map in the bracket. Note that this is zero indexed, so e.g. NM1 will have map_order 0, NM2 will have map_order 1, etc.
	map_order: number;
	/// The map's osu id. Note, that this is *not* the mapset id.
	map_id: number;
};
