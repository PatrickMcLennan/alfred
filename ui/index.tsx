import React from 'react';
import { App } from './App';
import { createRoot } from 'react-dom/client';

import './styles.css';
import 'swiper/css';
import 'swiper/css/navigation';
import 'swiper/css/pagination';

const ROOT = createRoot(document.querySelector('#ROOT') as HTMLDivElement);

ROOT.render(<App />);
