import React from "react";
import GlobalStateProvider from "./global_state";
import StateUser from "./state_consumer";

const App = () => {
	return (
		<div>
			<GlobalStateProvider>
				<StateUser />
			</GlobalStateProvider>
		</div>
	);
};

export default App;
