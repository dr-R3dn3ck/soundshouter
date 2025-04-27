// axios.js
import axios from 'axios';

// Set the default base URL for all Axios requests
axios.defaults.baseURL = 'http://localhost:8000/'; // Replace with your actual API base URL

// Optionally, set headers for authorization (if needed)
//axios.defaults.headers.common['Authorization'] = 'Bearer your-token'; // Replace with actual token if needed

export default axios;
