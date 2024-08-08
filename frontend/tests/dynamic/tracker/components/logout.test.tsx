/**
 * @jest-environment jsdom
 */

import { Logout } from 'src/dynamic/tracker/components/logout';

import React from 'react';

import '@testing-library/jest-dom';

import { render, screen } from '@testing-library/react';

test('test-Logout', () => {
  render(<Logout />);
  expect(screen.getByText("Log out")).toBeInTheDocument();
});
