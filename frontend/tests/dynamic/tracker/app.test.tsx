/**
 * @jest-environment jsdom
 */

import { App } from 'src/dynamic/tracker/app';

import React from 'react';

import '@testing-library/jest-dom';

import { render, screen } from '@testing-library/react';

test('test-App', () => {
  render(<App />);
  expect(screen.getByTestId("numbers")).toBeInTheDocument();
  expect(screen.getByText("OpenStreetMap")).toBeInTheDocument();
});
