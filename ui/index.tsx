import React from 'react';
import { App } from './App';
import { createRoot } from 'react-dom/client';

import './styles.css';
import 'swiper/css';
import 'swiper/css/controller';

const ROOT = createRoot(document.querySelector('#ROOT') as HTMLDivElement);

ROOT.render(<App />);
