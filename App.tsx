import React, { useState, useEffect } from 'react';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend } from 'recharts';
import axios from 'axios';

interface TechTrendData {
  language: string;
  trends: { date: string; popularity: number }[];
}

const TechTrendDashboard: React.FC = () => {
  const [trendData, setTrendData] = useState<TechTrendData[]>([]);
  const [searchQuery, setSearchQuery] = useState<string>('');
  const [error, setError] = useState<string | null>(null); 

  useEffect(() => {
    const fetchData = async () => {
      try {
        const response = await axios.get(`${process.env.REACT_APP_API_URL}/api/trends`);

        // Validate response data (you can enhance this validation as per your data structure)
        if (!Array.isArray(response.data)) {
          throw new Error('Invalid data format received');
        }
        setTrendData(response.data);

        // Clear any previous errors after successful data fetch
        if (error) {
          setError(null);
        }
      } catch (error) {
        if (axios.isAxiosError(error)) {
          console.error('Error fetching trend data:', error.message);
          setError('Failed to fetch trend data. Please try again later.'); // User-friendly error message
        } else if (error instanceof Error) {
          // Handle generic errors (e.g., invalid data format)
          console.error('An unexpected error occurred:', error.message);
          setError('An unexpected error occurred. Please try again.'); // Handle unexpected errors
        }
      }
    };

    fetchData();
    // Removing error from the dependency array to prevent infinite requests in case of persistent errors.
  }, []); // Removed error as a dependency to prevent infinite loop in case of persistent errors

  const handleSearchChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setSearchQuery(event.target.value);
  };

  const filteredTrends = trendData.filter(trend =>
    trend.language.toLowerCase().includes(searchQuery.toLowerCase())
  );

  return (
    <div>
      <h1>Tech Trend Dashboard</h1>
      <input
        type="text"
        placeholder="Search..."
        value={searchQuery}
        onChange={handleSearchChange}
      />
      {error && <div style={{ color: 'red' }}>{error}</div>}
      {filteredTrends.length > 0 ? (
        filteredTrends.map(trend => (
          <div key={trend.language}>
            <h2>{trend.language}</h2>
            <LineChart
              width={500}
              height={300}
              data={trend.trends}
              margin={{
                top: 5,
                right: 30,
                left: 20,
                bottom: 5,
              }}
            >
              <CartesianGrid strokeDasharray="3 3" />
              <XAxis dataKey="date" />
              <YAxis />
              <Tooltip />
              <Legend />
              <Line type="monotone" dataKey="popularity" stroke="#8884d8" />
            </LineChart>
          </div>
        ))
      ) : (
        <p>No trends match your search criteria.</p>
      )}
    </div>
  );
};

export default TechTrendDashboard;