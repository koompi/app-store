import React, { useContext, useEffect } from "react";
import { AuthContext } from "../context/AuthContext";
import { Modal, Button } from "antd";
import { GUEST, USER, DEVELOPER, ADMIN } from "../reducers/auth_reducer";

const AuthBox = (props) => {
	const { user, dispatch } = useContext(AuthContext);

	const { authModal, toggleAuthModal } = props;
	useEffect(() => console.log(user));
	return (
		<Modal
			title="Authentication Simulation"
			centered
			visible={authModal}
			onOk={() => toggleAuthModal(false)}
			onCancel={() => toggleAuthModal(false)}
		>
			<p>
				<Button onClick={() => dispatch({ type: GUEST })}>
					{GUEST}
				</Button>
			</p>
			<p>
				<Button onClick={() => dispatch({ type: USER })}>{USER}</Button>
			</p>
			<p>
				<Button onClick={() => dispatch({ type: DEVELOPER })}>
					{DEVELOPER}
				</Button>
			</p>
			<p>
				<Button onClick={() => dispatch({ type: ADMIN })}>
					{ADMIN}
				</Button>
			</p>
		</Modal>
	);
};

export default AuthBox;
