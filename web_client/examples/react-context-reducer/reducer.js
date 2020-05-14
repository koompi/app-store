export const ADD_TODO = "ADD_TODO";
export const REMOVE_TODO = "REMOVE_TODO";

export const reducer = ({ state, action }) => {
	switch (action.type) {
		case ADD_TODO:
			return [...state, { ...action.payload }];
		case REMOVE_TODO:
			return state.map((t, i) =>
				t.id === action.payload ? [...state.splice(i, 1)] : [...state]
			);
		default:
			return state;
	}
};
