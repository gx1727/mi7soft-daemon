import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  BarElement,
  Title,
  Tooltip,
  Legend,
} from 'chart.js';
import { Bar } from 'react-chartjs-2';

ChartJS.register(
  CategoryScale,
  LinearScale,
  BarElement,
  Title,
  Tooltip,
  Legend
);

const options = {
  responsive: true,
  plugins: {
    legend: {
      position: 'top' as const,
      labels: {
        color: '#9ca3af',
        font: {
          family: 'Inter',
        }
      }
    },
    title: {
      display: false,
    },
  },
  scales: {
    y: {
      grid: {
        color: 'rgba(255, 255, 255, 0.1)',
      },
      ticks: {
        color: '#9ca3af',
      }
    },
    x: {
      grid: {
        display: false,
      },
      ticks: {
        color: '#9ca3af',
      }
    }
  }
};

const labels = ['Startup Time (ms)', 'Memory Usage (MB)', 'CPU Overhead (%)'];

const data = {
  labels,
  datasets: [
    {
      label: 'MI7 Daemon',
      data: [12, 4.5, 0.1],
      backgroundColor: 'rgba(0, 212, 255, 0.8)',
    },
    {
      label: 'Standard Daemon',
      data: [150, 45, 2.5],
      backgroundColor: 'rgba(255, 255, 255, 0.1)',
    },
  ],
};

const PerformanceChart = () => {
  return <Bar options={options} data={data} />;
};

export default PerformanceChart;
