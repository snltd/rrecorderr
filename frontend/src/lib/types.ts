export interface Release {
  id: number;
  title: string;
  label_id: number;
  label_name: string;
  release_type: string;
  medium: string;
  colour: string;
  cat_no: string;
  released: number;
  notes: string;
  artists: Artist[];
}

export interface ValuedRelease {
  id: number;
  title: string;
  label_id: number;
  label_name: string;
  release_type: string;
  medium: string;
  released: number;
  artists: Artist[];
  value_estimated: number,
  value_cheapest_uk: number,
  value_cheapest_world: number,
}

export interface Artist {
  id: number;
  name: string;
}

export interface Label {
  id: number;
  name: string;
}
