import jwt from "jsonwebtoken";

const TokenSigner = async (role) => {
	let user_token = { email: "user@koompi.com", role: "user" };
	let maintainer_token = { email: "maintainer@koompi.com", role: "maintainer" };
	let admin_token = { email: "admin@koompi.com", role: "admin" };
	let secret = "helloworld";
	let token;
	switch (role) {
		case "user":
			token = await jwt.sign(user_token, secret);
			break;
		case "maintainer":
			token = await jwt.sign(maintainer_token, secret);
			break;
		case "admin":
			token = await jwt.sign(admin_token, secret);
			break;
		default:
			token = "null";
			break;
	}
	if (token === "null") {
		if (window.localStorage.getItem("token")) {
			window.localStorage.removeItem("token");
		}
	} else {
		window.localStorage.setItem("token", token);
	}
};

export default TokenSigner;
