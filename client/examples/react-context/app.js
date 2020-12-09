import React from "react";
import StateProvider from "./state";
import StateUser from "./state_user";

const App = () => {
	return (
		<div>
			<StateProvider>
				<StateUser />
			</StateProvider>
		</div>
	);
};

export default App;
