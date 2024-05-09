import React, { useState, useEffect } from 'react';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend } from 'recharts';
import axios from 'axios';

interface TechTrendData {
  language: string;
  trends: { date: string; popularity: number }[];
}

const TechTrendDashboard: React.FC = () => {
  const [trendData, setTrendData] = useState<TechTrendData[]>([]);
  const [searchQuery, setSearchQuery] = useState('');

  useEffect(() => {
    const fetchData = async () => {
      try {
        const response = await axios.get(`${process.env.REACT_APP_API_URL}/api/trends`);
        setTrendData(response.data);
      } catch (error) {
        console.error('Error fetching trend data:', error);
      }
    };

    fetchData();
  }, []);

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
      {filteredTrends.map(trend => (
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
      ))}
    </div>
  );
}

export default TechTrendDashboard;