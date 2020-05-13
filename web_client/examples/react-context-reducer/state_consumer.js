import React, { useContext, useState } from "react";
import uuid from "uuid/v1";

import GlobalStateContext from "./global_state";
import { ADD_TODO, REMOVE_TODO } from "./reducer";

const StateUser = (props) => {
	const [data, setData] = useState({ id: uuid(), text: "" });
	const { state, dispatch } = useContext(GlobalStateContext);
	return (
		<div>
			<form
				onSubmit={(e) => {
					e.preventDefault();
					dispatch({ type: ADD_TODO, payload: { ...data } });
				}}
			>
				<input value={data} onChange={(e) => setData(e.target.value)} />
			</form>

			{state.map((t) => (
				<div key={id}>
					{state.name}
					<button
						onClick={() => dispatch({ type: REMOVE_TODO, payload: { id } })}
					>
						REMOVE
					</button>
				</div>
			))}
		</div>
	);
};

export default StateUser;
