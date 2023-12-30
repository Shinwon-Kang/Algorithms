#include <iostream>
#include <vector>
#include <string>
#include <queue>

using namespace std;

// 최단거리 문제는 BFS

int main(void) {
	int N, M;
	cin >> N >> M;

	vector<vector<int>> map (N, vector<int> (M, 0));

	for (int i = 0; i < N; i++) {
		string s;
		cin >> s;
		for (int j = 0; j < s.size(); j++) {
			map[i][j] = s[j] - '0';
		}
	}
	map[0][0] = 0;

	queue<pair<vector<int>, int>> paths;
	vector<int> v1 = { 0, 0 };
	paths.push(pair<vector<int>, int>(v1, 1));
	
	int count;
	while (!paths.empty()) {
		pair<vector<int>, int> p = paths.front();
		paths.pop();

		int col = p.first[0];
		int row = p.first[1];

		count = p.second;

		if (col == N - 1 && row == M - 1) {
			cout << count;
			break;
		}

		if (col - 1 > -1 && map[col - 1][row] == 1) {
			map[col - 1][row] = 0;
			v1[0] = col - 1;
			v1[1] = row;
			paths.push(pair<vector<int>, int>(v1, count + 1));
		}

		if (col + 1 < map.size() && map[col + 1][row] == 1) {
			map[col + 1][row] = 0;
			v1[0] = col + 1;
			v1[1] = row;
			paths.push(pair<vector<int>, int>(v1, count + 1));
		}

		if (row - 1 > -1 && map[col][row - 1] == 1) {
			map[col][row - 1] = 0;
			v1[0] = col;
			v1[1] = row - 1;
			paths.push(pair<vector<int>, int>(v1, count + 1));
		}

		if (row + 1 < map[col].size() && map[col][row + 1] == 1) {
			map[col][row + 1] = 0;
			v1[0] = col;
			v1[1] = row + 1;
			paths.push(pair<vector<int>, int>(v1, count + 1));
		}

	}

	return 0;
}