import axios from 'axios';

export default axios.create({
	baseURL: "http://34.201.1.6:8000/",
	headers: {
		'Content-Type': 'application/json;charset=utf-8',
		'Accept': 'application/json'
	}
});



// export default axios.create({
// 	baseURL: "http://localhost:8000/",
// 	withCredentials: false,
// 	headers: {
// 		'Content-Type': 'application/json;charset=utf-8',
// 		'Accept': 'application/json',
// 		"Access-Control-Allow-Origin": "*",
// 		'Access-Control-Allow-Credentials':true
// 	}
// });