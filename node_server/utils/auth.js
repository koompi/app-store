const jwt = require("jsonwebtoken");
require("dotenv").config();
const { PRIVATE_KEY } = process.env;

const auth = (req, res, next) => {
	const { Authorization } = req.headers;
	if (Authorization) {
		try {
			const verified = jwt.verify(token, PRIVATE_KEY);
			req.user = verified;
			next();
		} catch (error) {
			res.status("401").json({
				status: "401",
				message: "Invalid authorization token",
			});
		}
	} else {
		res.status("401").json({
			status: "401",
			message: "Unauthorized",
		});
	}
};

module.exports = auth;
